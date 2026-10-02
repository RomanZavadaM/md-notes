import { useEffect, useMemo, useState } from "react";
import { api, type FieldSpec, type Note, type NoteTypeSpec, type SchemaDocument } from "../api";
import { useI18n } from "../i18n";
import "./PropertiesPanel.css";

interface Props {
  path: string;
  content: string;
  refreshKey: number;
  onContentChange: (content: string) => void;
}

type FormValues = Record<string, string | boolean>;

function frontMatterObject(note: Note | null): Record<string, unknown> {
  if (!note?.frontMatter || typeof note.frontMatter !== "object" || Array.isArray(note.frontMatter)) return {};
  return note.frontMatter as Record<string, unknown>;
}

function displayValue(value: unknown, spec: FieldSpec): string | boolean {
  if (spec.type === "boolean") return value === true;
  if (spec.type === "list" || spec.type === "links") {
    if (Array.isArray(value)) return value.filter((item) => typeof item === "string").join(", ");
    return typeof value === "string" ? value : "";
  }
  return value == null ? "" : String(value);
}

function toPatchValue(value: string | boolean, spec: FieldSpec, required: boolean): unknown {
  if (spec.type === "boolean") return Boolean(value);
  const text = String(value).trim();
  if (!text && !required) return null;
  if (spec.type === "number") {
    const number = Number(text);
    return Number.isFinite(number) ? number : text;
  }
  if (spec.type === "list" || spec.type === "links") {
    return text ? text.split(",").map((item) => item.trim().replace(/^#/, "")).filter(Boolean) : null;
  }
  return text;
}

export function PropertiesPanel({ path, content, refreshKey, onContentChange }: Props) {
  const { t } = useI18n();
  const [schema, setSchema] = useState<SchemaDocument | null>(null);
  const [parsed, setParsed] = useState<Note | null>(null);
  const [selectedType, setSelectedType] = useState("");
  const [values, setValues] = useState<FormValues>({});
  const [saving, setSaving] = useState(false);
  const [message, setMessage] = useState("");
  const [error, setError] = useState("");

  const frontMatter = useMemo(() => frontMatterObject(parsed), [parsed]);
  const currentType = typeof frontMatter.type === "string" ? frontMatter.type : "";

  useEffect(() => {
    api.getSchema().then(setSchema).catch((reason) => {
      setSchema(null);
      setError(String(reason));
    });
  }, [refreshKey]);

  useEffect(() => {
    let cancelled = false;
    const timer = window.setTimeout(() => {
      api.parseNoteContent(path, content)
        .then((note) => {
          if (!cancelled) {
            setParsed(note);
            setError("");
          }
        })
        .catch((reason) => {
          if (!cancelled) setError(String(reason));
        });
    }, 120);
    return () => {
      cancelled = true;
      window.clearTimeout(timer);
    };
  }, [path, content]);

  useEffect(() => {
    setSelectedType(currentType);
    setMessage("");
  }, [path, currentType]);

  const typeSpec: NoteTypeSpec | undefined = schema?.types[selectedType];

  useEffect(() => {
    if (!schema || !typeSpec) {
      setValues({});
      return;
    }
    const next: FormValues = {};
    for (const name of typeSpec.fields) {
      const spec = schema.fields[name];
      if (spec) next[name] = displayValue(frontMatter[name], spec);
    }
    setValues(next);
  }, [schema, typeSpec, frontMatter, selectedType]);

  if (error) return <div className="properties-empty warn">{error}</div>;
  if (!parsed) return <div className="properties-empty">{t.propertyNoNote}</div>;
  if (parsed.frontMatterError) {
    return <div className="properties-empty warn">{t.propertyInvalidFrontMatter(parsed.frontMatterError)}</div>;
  }
  if (!schema || Object.keys(schema.types).length === 0) {
    return (
      <div className="properties-empty">
        <p>{t.propertyNoSchema}</p>
        <code>.mdnotes/schema.json</code>
      </div>
    );
  }

  const unknownType = selectedType !== "" && !schema.types[selectedType];

  const apply = async () => {
    const patch: Record<string, unknown> = { type: selectedType || null };
    if (typeSpec) {
      for (const name of typeSpec.fields) {
        const spec = schema.fields[name];
        if (!spec || spec.readonly) continue;
        patch[name] = toPatchValue(values[name] ?? "", spec, typeSpec.required.includes(name));
      }
    }
    setSaving(true);
    setMessage("");
    setError("");
    try {
      const formatted = await api.formatNoteProperties(content, patch);
      onContentChange(formatted);
      setParsed(await api.parseNoteContent(path, formatted));
      setMessage(t.propertySaved);
    } catch (reason) {
      setError(String(reason));
    } finally {
      setSaving(false);
    }
  };

  return (
    <div className="properties-panel">
      <div className="properties-schema-hint">{t.propertySchemaHint}</div>
      <label className="property-field">
        <span>{t.propertyType}</span>
        <select value={selectedType} onChange={(event) => setSelectedType(event.target.value)}>
          <option value="">{t.propertyNoType}</option>
          {unknownType && <option value={selectedType}>{t.propertyUnknownType(selectedType)}</option>}
          {Object.entries(schema.types).map(([name, spec]) => (
            <option key={name} value={name}>
              {spec.label || name}
            </option>
          ))}
        </select>
      </label>

      {unknownType && <p className="properties-note">{t.propertyUnknownTypeHelp}</p>}

      {typeSpec && typeSpec.fields.map((name) => {
        const spec = schema.fields[name];
        if (!spec) return null;
        return (
          <PropertyField
            key={name}
            name={name}
            spec={spec}
            required={typeSpec.required.includes(name)}
            value={values[name] ?? (spec.type === "boolean" ? false : "")}
            onChange={(value) => setValues((current) => ({ ...current, [name]: value }))}
          />
        );
      })}

      <div className="properties-actions">
        <button type="button" className="primary" disabled={saving || unknownType} onClick={() => void apply()}>
          {saving ? t.propertySaving : t.propertyApply}
        </button>
        {message && <span className="properties-saved">{message}</span>}
      </div>
    </div>
  );
}

function PropertyField({
  name,
  spec,
  required,
  value,
  onChange,
}: {
  name: string;
  spec: FieldSpec;
  required: boolean;
  value: string | boolean;
  onChange: (value: string | boolean) => void;
}) {
  const { t } = useI18n();
  const label = spec.label || name;
  const suffix = required ? ` · ${t.propertyRequired}` : "";

  if (spec.type === "boolean") {
    return (
      <label className="property-field property-checkbox">
        <input
          type="checkbox"
          checked={Boolean(value)}
          disabled={spec.readonly}
          onChange={(event) => onChange(event.target.checked)}
        />
        <span>{label}{suffix}</span>
      </label>
    );
  }

  if (spec.type === "enum") {
    return (
      <label className="property-field">
        <span>{label}{suffix}</span>
        <select
          value={String(value)}
          disabled={spec.readonly}
          onChange={(event) => onChange(event.target.value)}
        >
          {!required && <option value="">{t.propertyEmpty}</option>}
          {spec.values.map((option) => <option key={option} value={option}>{option}</option>)}
        </select>
      </label>
    );
  }

  const inputType =
    spec.type === "number" ? "number" : spec.type === "date" ? "date" : spec.type === "url" ? "url" : "text";
  const listLike = spec.type === "list" || spec.type === "links";
  return (
    <label className="property-field">
      <span>{label}{suffix}</span>
      <input
        type={inputType}
        value={String(value)}
        disabled={spec.readonly}
        placeholder={listLike ? t.propertyTagsPlaceholder : undefined}
        onChange={(event) => onChange(event.target.value)}
      />
    </label>
  );
}
