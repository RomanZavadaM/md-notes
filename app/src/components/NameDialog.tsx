import { useState } from "react";

export interface Choice {
  value: string;
  label: string;
}

interface Props {
  title: string;
  label: string;
  initial: string;
  submitText: string;
  /** Optional drop-down shown under the name, e.g. note templates. */
  choices?: Choice[];
  choiceLabel?: string;
  onSubmit: (value: string, choice: string) => void;
  onCancel: () => void;
}

/** Small modal with a text field: names for new notes, folders, renames. */
export function NameDialog({ title, label, initial, submitText, choices, choiceLabel, onSubmit, onCancel }: Props) {
  const [value, setValue] = useState(initial);
  const [choice, setChoice] = useState(choices?.[0]?.value ?? "");
  const trimmed = value.trim();

  return (
    <div className="dialog-backdrop" onMouseDown={onCancel}>
      <form
        className="dialog"
        role="dialog"
        aria-label={title}
        onMouseDown={(e) => e.stopPropagation()}
        onSubmit={(e) => {
          e.preventDefault();
          if (trimmed) onSubmit(trimmed, choice);
        }}
        onKeyDown={(e) => {
          if (e.key === "Escape") onCancel();
        }}
      >
        <h2>{title}</h2>
        <label>
          {label}
          <input autoFocus value={value} onChange={(e) => setValue(e.target.value)} onFocus={(e) => e.target.select()} />
        </label>
        {choices && choices.length > 1 && (
          <label>
            {choiceLabel}
            <select value={choice} onChange={(e) => setChoice(e.target.value)}>
              {choices.map((c) => (
                <option key={c.value} value={c.value}>
                  {c.label}
                </option>
              ))}
            </select>
          </label>
        )}
        <div className="dialog-actions">
          <button type="button" onClick={onCancel}>
            Скасувати
          </button>
          <button type="submit" className="primary" disabled={!trimmed}>
            {submitText}
          </button>
        </div>
      </form>
    </div>
  );
}
