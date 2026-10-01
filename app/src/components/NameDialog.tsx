import { useState } from "react";

interface Props {
  title: string;
  label: string;
  initial: string;
  submitText: string;
  onSubmit: (value: string) => void;
  onCancel: () => void;
}

/** Small modal with one text field: names for new notes, folders, renames. */
export function NameDialog({ title, label, initial, submitText, onSubmit, onCancel }: Props) {
  const [value, setValue] = useState(initial);
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
          if (trimmed) onSubmit(trimmed);
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
