import { useEffect, useState, type ImgHTMLAttributes } from "react";
import { convertFileSrc } from "@tauri-apps/api/core";
import { api } from "../api";

function absoluteVaultPath(vaultRoot: string, rel: string): string {
  const sep = vaultRoot.includes("\\") ? "\\" : "/";
  return `${vaultRoot}${sep}${rel.split("/").join(sep)}`;
}

export function mimeTypeForPath(path: string): string {
  const ext = path.split(".").pop()?.toLowerCase();
  switch (ext) {
    case "png": return "image/png";
    case "jpg":
    case "jpeg": return "image/jpeg";
    case "gif": return "image/gif";
    case "webp": return "image/webp";
    case "svg": return "image/svg+xml";
    case "bmp": return "image/bmp";
    case "avif": return "image/avif";
    case "pdf": return "application/pdf";
    default: return "application/octet-stream";
  }
}

export function useVaultAssetUrl(
  path: string | null,
  vaultRoot: string,
  encrypted: boolean,
  onError?: (error: unknown) => void,
): string | null {
  const [url, setUrl] = useState<string | null>(null);

  useEffect(() => {
    if (!path) {
      setUrl(null);
      return;
    }
    if (!encrypted) {
      setUrl(convertFileSrc(absoluteVaultPath(vaultRoot, path)));
      return;
    }

    let cancelled = false;
    let objectUrl: string | null = null;
    setUrl(null);
    api.readVaultFile(path)
      .then((buffer) => {
        if (cancelled) return;
        objectUrl = URL.createObjectURL(new Blob([buffer], { type: mimeTypeForPath(path) }));
        setUrl(objectUrl);
      })
      .catch((error) => {
        if (!cancelled) onError?.(error);
      });

    return () => {
      cancelled = true;
      if (objectUrl) URL.revokeObjectURL(objectUrl);
    };
  }, [encrypted, onError, path, vaultRoot]);

  return url;
}

interface VaultImageProps extends Omit<ImgHTMLAttributes<HTMLImageElement>, "src"> {
  path: string;
  vaultRoot: string;
  encrypted: boolean;
  onAssetError?: (error: unknown) => void;
}

export function VaultImage({ path, vaultRoot, encrypted, onAssetError, ...props }: VaultImageProps) {
  const url = useVaultAssetUrl(path, vaultRoot, encrypted, onAssetError);
  return url ? <img {...props} src={url} /> : null;
}

export function VaultPdfFrame({
  path,
  vaultRoot,
  encrypted,
  title,
  className,
  onAssetError,
}: {
  path: string;
  vaultRoot: string;
  encrypted: boolean;
  title: string;
  className?: string;
  onAssetError?: (error: unknown) => void;
}) {
  const url = useVaultAssetUrl(path, vaultRoot, encrypted, onAssetError);
  return url ? <iframe className={className} src={url} title={title} /> : null;
}
