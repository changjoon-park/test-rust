// src/types/modal.ts
export interface ModalRoute {
  component: React.ComponentType<any>;
  title: string;
}

export type ModalRoutes = Record<string, ModalRoute>;
