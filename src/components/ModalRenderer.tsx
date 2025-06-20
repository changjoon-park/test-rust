// src/components/ModalRenderer.tsx
"use client";

import { Suspense } from "react";
import { useModalRouter } from "@/hooks/useModalRouter";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";

function ModalContent() {
  const { currentModal, closeModal } = useModalRouter();

  if (!currentModal) return null;

  const { component: ModalComponent, title, params } = currentModal;

  return (
    <Dialog 
      open={true} 
      onOpenChange={(open) => {
        if (!open) {
          closeModal();
        }
      }}
    >
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{title}</DialogTitle>
        </DialogHeader>
        <Suspense fallback={<div>Loading...</div>}>
          <ModalComponent {...params} />
        </Suspense>
      </DialogContent>
    </Dialog>
  );
}

export function ModalRenderer() {
  return (
    <Suspense fallback={null}>
      <ModalContent />
    </Suspense>
  );
}
