// src/lib/modalRoutes.ts
import { lazy } from "react";
import type { ModalRoutes } from "@/types/modal";

export const modalRoutes: ModalRoutes = {
  demo: {
    component: lazy(() => import("@/components/modals/DemoModal")),
    title: "Demo Modal",
  },
  settings: {
    component: lazy(() => import("@/components/modals/SettingsModal")),
    title: "Settings",
  },
};
