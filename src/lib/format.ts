/**
 * Shared formatting utilities.
 */

/** Human-readable duration, e.g. "3m 12s" or "1h 5m". */
export function formatDuration(secs: number): string {
  const h = Math.floor(secs / 3600);
  const m = Math.floor((secs % 3600) / 60);
  const s = Math.floor(secs % 60);
  if (h > 0) return `${h}h ${m}m ${s}s`;
  if (m > 0) return `${m}m ${s}s`;
  return `${s}s`;
}

/** Zero-padded timer string, e.g. "00:03:12". */
export function formatTimer(secs: number): string {
  const h = Math.floor(secs / 3600);
  const m = Math.floor((secs % 3600) / 60);
  const s = Math.floor(secs % 60);
  return [h, m, s].map((v) => String(v).padStart(2, "0")).join(":");
}

/** Convert raw error to user-friendly Portuguese message. */
export function formatError(e: unknown): string {
  const raw = String(e);
  console.error("[hlusra]", raw);

  // Already a friendly message from backend
  if (raw.startsWith("Falha") || raw.startsWith("Nenhuma") || raw.startsWith("Erro")) {
    return raw;
  }

  // Known patterns
  if (raw.includes("StateChangeError")) return "Falha ao iniciar pipeline de gravação. Verifique se o PipeWire está rodando.";
  if (raw.includes("lock poisoned")) return "Erro interno do aplicativo. Reinicie o Hlusra.";
  if (raw.includes("No active recording")) return "Nenhuma gravação ativa.";
  if (raw.includes("not configured")) return "Configure as chaves de API nas Configurações.";
  if (raw.includes("not found") || raw.includes("NotFound")) return "Reunião não encontrada.";
  if (raw.includes("ffmpeg")) return "FFmpeg não encontrado. Instale com: sudo pacman -S ffmpeg";
  if (raw.includes("whisper") || raw.includes("model")) return "Erro no modelo de transcrição. Verifique o modelo selecionado.";
  if (raw.includes("network") || raw.includes("reqwest")) return "Erro de rede. Verifique sua conexão.";
  if (raw.includes("permission") || raw.includes("denied")) return "Permissão negada. Verifique as permissões do diretório.";

  // Fallback
  return `Erro: ${raw.length > 150 ? raw.slice(0, 150) + "..." : raw}`;
}
