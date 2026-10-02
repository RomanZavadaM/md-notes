import { useEffect, useMemo, useState } from "react";
import { api, type Note, type NoteTypeSpec, type PropertySpec, type SchemaDocument } from "../api";
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

function displayValue(value: unknown, spec: PropertySpec): string | boolean {
  if (spec.type === "boolean") return value === true;
  if (spec.type === "tags") {
    if (Array.isArray(value)) return value.filter((item) => typeof item === "string").join(", ");
    return typeof value === "string" ? value : "";
  }
  return value == null ? "" : String(value);
}

function toPatchValue(value: string | boolean, spec: PropertySpec): unknown {
  if (spec.type === "boolean") return Boolean(value);
  const text = String(value).trim();
  if (!text && !spec.required) return null;
  if (spec.type === "number") {
    const number = Number(text);
    return Number.isFinite(number) ? number : text;
  }
  if (spec.type === "tags") {
    return text ? text.split(",").map((tag) => tag.trim().replace(/^#/, "")).filter(Boolean) : null;
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
    if (!typeSpec) {
      setValues({});
      return;
    }
    const next: FormValues = {};
    for (const [name, spec] of Object.entries(typeSpec.properties)) {
      next[name] = displayValue(frontMatter[name], spec);
    }
    setValues(next);
  }, [typeSpec, frontMatter, selectedType]);

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
      for (const [name, spec] of Object.entries(typeSpec.properties)) {
        patch[name] = toPatchValue(values[name] ?? "", spec);
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

      {typeSpec &&
        Object.entries(typeSpec.properties).map(([name, spec]) => (
          <PropertyField
            key={name}
            name={name}
            spec={spec}
            value={values[name] ?? (spec.type === "boolean" ? false : "")}
            onChange={(value) => setValues((current) => ({ ...current, [name]: value }))}
          />
        ))}

      <div className="properties-actions">
        <button type="button" className="primary" disabled={saving} onClick={() => void apply()}>
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
  value,
  onChange,
}: {
  name: string;
  spec: PropertySpec;
  value: string | boolean;
  onChange: (value: string | boolean) => void;
}) {
  const { t } = useI18n();
  const label = spec.label || name;
  const required = spec.required ? ` · ${t.propertyRequired}` : "";

  if (spec.type === "boolean") {
    return (
      <label className="property-field property-checkbox">
        <input type="checkbox" checked={Boolean(value)} onChange={(event) => onChange(event.target.checked)} />
        <span>{label}{required}</span>
      </label>
    );
  }

  if (spec.type === "select") {
    return (
      <label className="property-field">
        <span>{label}{required}</span>
        <select value={String(value)} onChange={(event) => onChange(event.target.value)}>
          {!spec.required && <option value="">{t.propertyEmpty}</option>}
          {spec.options.map((option) => <option key={option} value={option}>{option}</option>)}
        </select>
      </label>
    );
  }

  if (spec.type === "text") {
    return (
      <label className="property-field">
        <span>{label}{required}</span>
        <textarea rows={3} value={String(value)} onChange={(event) => onChange(event.target.value)} />
      </label>
    );
  }

  const inputType = spec.type === "number" ? "number" : spec.type === "date" ? "date" : "text";
  return (
    <label className="property-field">
      <span>{label}{required}</span>
      <input
        type={inputType}
        value={String(value)}
        placeholder={spec.type === "tags" ? t.propertyTagsPlaceholder : undefined}
        onChange={(event) => onChange(event.target.value)}
      />
    </label>
  );
}
