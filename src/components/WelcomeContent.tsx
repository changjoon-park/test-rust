// src/components/WelcomeContent.tsx
"use client";

import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Separator } from "@/components/ui/separator";
import { Progress } from "@/components/ui/progress";
import { useTauriInvoke } from "@/hooks/useTauriInvoke";
import { useTauriInvokeWithProgress } from "@/hooks/useTauriInvokeWithProgress";
import { useModalRouter } from "@/hooks/useModalRouter";
import {
  Clock,
  Zap,
  Shield,
  Rocket,
  Activity,
  Layers,
  LogOut,
} from "lucide-react";
import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";

export function WelcomeContent() {
  const { data: greeting, isLoading, execute: greet } = useTauriInvoke("greet");

  const {
    data: systemInfo,
    isLoading: isAnalyzing,
    execute: analyzeSystem,
    progress,
    progressMessage,
  } = useTauriInvokeWithProgress("analyze_system");

  const { openModal, navigateToModal } = useModalRouter();
  const [isLoggingOut, setIsLoggingOut] = useState(false);

  const handleLogout = async () => {
    try {
      setIsLoggingOut(true);
      await invoke("switch_to_login_window");
    } catch (error) {
      console.error("Logout failed:", error);
      setIsLoggingOut(false);
    }
  };

  const features = [
    {
      icon: <Zap className="h-5 w-5" />,
      title: "Lightning Fast",
      description: "Powered by Rust and WebView",
    },
    {
      icon: <Shield className="h-5 w-5" />,
      title: "Secure by Default",
      description: "Sandboxed environment with minimal permissions",
    },
    {
      icon: <Rocket className="h-5 w-5" />,
      title: "Cross Platform",
      description: "Windows, macOS, and Linux support",
    },
  ];

  return (
    <>
      {/* Header with Logout button */}
      <div className="flex justify-between items-center mb-8">
        <div>
          <h2 className="text-2xl font-semibold">Dashboard</h2>
          <p className="text-muted-foreground">Welcome back to Monori</p>
        </div>
        <Button
          onClick={() => {
            void handleLogout();
          }}
          disabled={isLoggingOut}
          variant="outline"
          size="sm"
        >
          <LogOut className="mr-2 h-4 w-4" />
          {isLoggingOut ? "Logging out..." : "Logout"}
        </Button>
      </div>

      {/* Main Card - existing greet */}
      <Card className="mb-8">
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Clock className="h-5 w-5" />
            Test Tauri Integration
          </CardTitle>
          <CardDescription>
            Click the button below to invoke a Rust function
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="flex flex-col sm:flex-row gap-4 items-start sm:items-center">
            <Button
              onClick={() => {
                void greet();
              }}
              disabled={isLoading}
              size="lg"
            >
              {isLoading ? "Calling Rust..." : "Get Current Time"}
            </Button>

            {greeting && (
              <div className="flex-1">
                <Badge variant="secondary" className="text-sm p-2">
                  {greeting}
                </Badge>
              </div>
            )}
          </div>
        </CardContent>
      </Card>

      {/* Progress Demo Card */}
      <Card className="mb-8">
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Activity className="h-5 w-5" />
            System Analysis with Progress
          </CardTitle>
          <CardDescription>
            See real-time progress updates from Rust
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-4">
            <Button
              onClick={() => {
                void analyzeSystem();
              }}
              disabled={isAnalyzing}
              size="lg"
              variant="outline"
            >
              {isAnalyzing ? "Analyzing..." : "Analyze System"}
            </Button>

            {isAnalyzing && (
              <div className="space-y-2">
                <Progress value={progress} className="w-full" />
                <p className="text-sm text-muted-foreground">
                  {progressMessage ?? `${progress}% 완료`}
                </p>
              </div>
            )}

            {systemInfo && !isAnalyzing && (
              <div className="p-3 bg-muted rounded-md">
                <p className="text-sm">{systemInfo}</p>
              </div>
            )}
          </div>

          <Separator />

          <div className="space-y-2 text-sm text-muted-foreground">
            <p>
              This demonstrates real-time progress updates using Tauri&apos;s
              event system for long-running operations.
            </p>
          </div>
        </CardContent>
      </Card>

      {/* Modal Route Demo Card */}
      <Card className="mb-8">
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Layers className="h-5 w-5" />
            Modal Route Pattern
          </CardTitle>
          <CardDescription>
            URL-based modal state management for better UX
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
            <div className="space-y-2">
              <h4 className="text-sm font-semibold">Quick Open (Replace)</h4>
              <Button
                onClick={() => {
                  openModal("demo", { id: "123" });
                }}
                variant="outline"
                className="w-full"
              >
                Open Demo Modal
              </Button>
              <p className="text-xs text-muted-foreground">
                Opens without history entry
              </p>
            </div>

            <div className="space-y-2">
              <h4 className="text-sm font-semibold">Navigate (History)</h4>
              <Button
                onClick={() => {
                  navigateToModal("settings");
                }}
                variant="outline"
                className="w-full"
              >
                Open Settings
              </Button>
              <p className="text-xs text-muted-foreground">
                Adds to browser history
              </p>
            </div>
          </div>

          <Separator />

          <div className="space-y-2 text-sm text-muted-foreground">
            <p>
              Modal Route pattern enables shareable URLs, browser back button
              support, and state persistence across refreshes.
            </p>
            <p className="text-xs">
              Try opening a modal and then refreshing the page - the modal will
              remain open!
            </p>
          </div>
        </CardContent>
      </Card>

      {/* Features Grid */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-12">
        {features.map((feature, index) => (
          <Card key={index}>
            <CardHeader>
              <CardTitle className="flex items-center gap-2 text-lg">
                {feature.icon}
                {feature.title}
              </CardTitle>
            </CardHeader>
            <CardContent>
              <p className="text-sm text-muted-foreground">
                {feature.description}
              </p>
            </CardContent>
          </Card>
        ))}
      </div>

      {/* Quick Start */}
      <Card>
        <CardHeader>
          <CardTitle>Quick Start</CardTitle>
          <CardDescription>
            Get up and running with your Tauri app
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-2">
            <h3 className="font-semibold">Development</h3>
            <code className="block bg-muted p-3 rounded-md text-sm">
              pnpm tauri dev
            </code>
          </div>

          <div className="space-y-2">
            <h3 className="font-semibold">Build for Production</h3>
            <code className="block bg-muted p-3 rounded-md text-sm">
              pnpm tauri build
            </code>
          </div>

          <Separator />

          <div className="flex flex-wrap gap-2">
            <Badge>Rust {">"}= 1.70</Badge>
            <Badge>Node.js {">"}= 18</Badge>
            <Badge>Tauri 2.0</Badge>
            <Badge>Next.js 15</Badge>
          </div>
        </CardContent>
      </Card>
    </>
  );
}
