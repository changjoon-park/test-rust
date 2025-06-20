// src/app/page.tsx
import { Suspense } from "react";
import { WelcomeContent } from "@/components/WelcomeContent";

export default function WelcomePage() {
  return (
    <div className="min-h-screen bg-background">
      <div className="container mx-auto px-4 py-16 max-w-4xl">
        {/* Header */}
        <div className="text-center mb-12">
          <h1 className="text-4xl font-bold tracking-tight mb-4">
            Welcome to Tauri + Next.js
          </h1>
          <p className="text-xl text-muted-foreground">
            Build smaller, faster, and more secure desktop applications
          </p>
        </div>

        {/* Wrap the content in Suspense */}
        <Suspense fallback={
          <div className="text-center py-8">
            <p className="text-muted-foreground">Loading...</p>
          </div>
        }>
          <WelcomeContent />
        </Suspense>

        {/* Footer */}
        <footer className="mt-12 text-center text-sm text-muted-foreground">
          <p>Built with ❤️ using Tauri, Next.js, and shadcn/ui</p>
        </footer>
      </div>
    </div>
  );
}
