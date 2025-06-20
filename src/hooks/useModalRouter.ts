// src/hooks/useModalRouter.ts
"use client";

import { useSearchParams, useRouter, usePathname } from "next/navigation";
import { useCallback, useMemo } from "react";
import { modalRoutes } from "@/lib/modalRoutes";

export function useModalRouter() {
  const searchParams = useSearchParams();
  const router = useRouter();
  const pathname = usePathname();

  const currentModal = useMemo(() => {
    const modalName = searchParams.get("modal");
    if (!modalName) return null;
    
    // Check if modalName exists in modalRoutes
    if (!(modalName in modalRoutes)) return null;

    return {
      name: modalName,
      component: modalRoutes[modalName].component,
      title: modalRoutes[modalName].title,
      params: Object.fromEntries(searchParams.entries()),
    };
  }, [searchParams]);

  const openModal = useCallback(
    (modalName: string, params?: Record<string, string>) => {
      const newParams = new URLSearchParams(searchParams);
      newParams.set("modal", modalName);
      
      if (params) {
        Object.entries(params).forEach(([key, value]) => {
          newParams.set(key, value);
        });
      }

      window.history.replaceState(null, "", `${pathname}?${newParams.toString()}`);
    },
    [pathname, searchParams]
  );

  const navigateToModal = useCallback(
    (modalName: string, params?: Record<string, string>) => {
      const newParams = new URLSearchParams(searchParams);
      newParams.set("modal", modalName);
      
      if (params) {
        Object.entries(params).forEach(([key, value]) => {
          newParams.set(key, value);
        });
      }

      router.push(`${pathname}?${newParams.toString()}`);
    },
    [pathname, router, searchParams]
  );

  const closeModal = useCallback(() => {
    const newParams = new URLSearchParams(searchParams);
    newParams.delete("modal");
    
    // Remove modal-specific params
    Array.from(searchParams.keys()).forEach((key) => {
      if (key !== "modal") {
        newParams.delete(key);
      }
    });

    router.push(`${pathname}${newParams.toString() ? `?${newParams.toString()}` : ""}`);
  }, [pathname, router, searchParams]);

  return {
    currentModal,
    openModal,
    navigateToModal,
    closeModal,
  };
}
