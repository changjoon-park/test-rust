// src/hooks/useTauriInvoke.ts
import { invoke, type InvokeArgs } from "@tauri-apps/api/core";
import { useState, useCallback } from "react";
import type { TauriCommands } from "@/types/commands";

interface UseTauriInvokeOptions {
  onSuccess?: (data: unknown) => void;
  onError?: (error: unknown) => void;
}

export function useTauriInvoke<K extends keyof TauriCommands>(
  command: K,
  options?: UseTauriInvokeOptions,
) {
  type Args = TauriCommands[K]['args'];
  type Result = TauriCommands[K]['result'];

  const [data, setData] = useState<Result | null>(null);
  const [error, setError] = useState<Error | null>(null);
  const [isLoading, setIsLoading] = useState(false);

  const execute = useCallback(
    async (args?: Args) => {
      setIsLoading(true);
      setError(null);

      try {
        // void 타입을 처리하기 위한 조건부 로직
        const result = args === undefined 
          ? await invoke<Result>(command)
          : await invoke<Result>(command, args as InvokeArgs);
        
        setData(result);
        options?.onSuccess?.(result);
        return result;
      } catch (err) {
        const error = err instanceof Error ? err : new Error(String(err));
        setError(error);
        options?.onError?.(error);
        throw error;
      } finally {
        setIsLoading(false);
      }
    },
    [command, options],
  );

  return { data, error, isLoading, execute };
}
