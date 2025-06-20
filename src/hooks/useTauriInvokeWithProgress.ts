// src/hooks/useTauriInvokeWithProgress.ts
import { invoke, type InvokeArgs } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { useState, useCallback, useEffect, useRef } from "react";
import type { TauriCommands } from "@/types/commands";

interface ProgressEvent {
  value: number;
  message?: string;
}

export function useTauriInvokeWithProgress<K extends keyof TauriCommands>(
  command: K,
  options?: {
    onSuccess?: (data: unknown) => void;
    onError?: (error: unknown) => void;
    onProgress?: (progress: ProgressEvent) => void;
  },
) {
  type Args = TauriCommands[K]["args"];
  type Result = TauriCommands[K]["result"];

  const [data, setData] = useState<Result | null>(null);
  const [error, setError] = useState<Error | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [progress, setProgress] = useState<ProgressEvent>({ value: 0 });

  const unlistenRef = useRef<UnlistenFn | null>(null);

  // 이벤트 리스너 설정
  useEffect(() => {
    const setupListener = async () => {
      unlistenRef.current = await listen<ProgressEvent>(
        `${command}_progress`,
        (event) => {
          setProgress(event.payload);
          options?.onProgress?.(event.payload);
        },
      );
    };

    void setupListener();

    return () => {
      if (unlistenRef.current) {
        unlistenRef.current();
      }
    };
  }, [command, options]);

  const execute = useCallback(
    async (args?: Args) => {
      setIsLoading(true);
      setError(null);
      setProgress({ value: 0 }); // 진행률 초기화

      try {
        const result =
          args === undefined
            ? await invoke<Result>(command)
            : await invoke<Result>(command, args as InvokeArgs);

        setData(result);
        setProgress({ value: 100 }); // 완료
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

  return {
    data,
    error,
    isLoading,
    execute,
    progress: progress.value,
    progressMessage: progress.message,
  };
}
