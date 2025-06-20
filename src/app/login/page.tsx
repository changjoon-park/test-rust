// src/app/login/page.tsx
"use client";

import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Alert, AlertDescription } from "@/components/ui/alert";
import { Loader2, LogIn, AlertCircle, User, Key } from "lucide-react";

export default function LoginPage() {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fillTestCredentials = () => {
    setEmail("test@example.com");
    setPassword("password123");
    setError(null);
  };

  const handleLogin = async (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);
    setIsLoading(true);

    try {
      // Trim inputs to remove any whitespace
      const trimmedEmail = email.trim();
      const trimmedPassword = password.trim();

      // Validate inputs
      if (!trimmedEmail || !trimmedPassword) {
        throw new Error("Please enter both email and password");
      }

      // Debug logging
      console.log("Attempting login with:", {
        email: trimmedEmail,
        password: trimmedPassword,
      });

      // Validate credentials with backend
      const token = await invoke<string>("validate_login", {
        email: trimmedEmail,
        password: trimmedPassword,
      });

      console.log("Login successful, token received:", token);

      // Save session in Tauri
      await invoke("save_session", { token });
      console.log("Session saved successfully");

      // Switch to main window
      console.log("Switching to main window...");
      await invoke("switch_to_main_window");
      console.log("Switch to main window completed");

      // No need to redirect - the window will change
    } catch (err) {
      console.error("Login failed:", err);
      console.error("Error details:", JSON.stringify(err));

      // Check if it's a Tauri error
      if (err && typeof err === "object" && "message" in err) {
        setError(err.message as string);
      } else if (err instanceof Error) {
        setError(err.message);
      } else {
        setError("Login failed. Please try again.");
      }
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="min-h-screen bg-background flex items-center justify-center p-4">
      <Card className="w-full max-w-md">
        <CardHeader className="space-y-1">
          <CardTitle className="text-2xl font-bold text-center">
            Welcome to Monori
          </CardTitle>
          <CardDescription className="text-center">
            Sign in to your account to continue
          </CardDescription>
        </CardHeader>
        <form
          onSubmit={(e) => {
            void handleLogin(e);
          }}
        >
          <CardContent className="space-y-4">
            {error && (
              <Alert variant="destructive">
                <AlertCircle className="h-4 w-4" />
                <AlertDescription>{error}</AlertDescription>
              </Alert>
            )}

            <div className="space-y-2">
              <Label htmlFor="email">Email</Label>
              <Input
                id="email"
                type="email"
                placeholder="name@example.com"
                value={email}
                onChange={(e) => {
                  setEmail(e.target.value);
                }}
                disabled={isLoading}
                required
              />
            </div>

            <div className="space-y-2">
              <Label htmlFor="password">Password</Label>
              <Input
                id="password"
                type="password"
                placeholder="Enter your password"
                value={password}
                onChange={(e) => {
                  setPassword(e.target.value);
                }}
                disabled={isLoading}
                required
              />
            </div>

            <Card className="bg-muted/50">
              <CardContent className="pt-6">
                <div className="space-y-3">
                  <h4 className="text-sm font-semibold flex items-center gap-2">
                    <User className="h-4 w-4" />
                    Test Account Credentials
                  </h4>
                  <div className="space-y-2 text-sm">
                    <div className="flex items-center gap-2">
                      <span className="text-muted-foreground">Email:</span>
                      <code className="px-2 py-1 bg-background rounded text-xs">
                        test@example.com
                      </code>
                    </div>
                    <div className="flex items-center gap-2">
                      <span className="text-muted-foreground">Password:</span>
                      <code className="px-2 py-1 bg-background rounded text-xs">
                        password123
                      </code>
                    </div>
                  </div>
                  <Button
                    type="button"
                    variant="secondary"
                    size="sm"
                    className="w-full mt-3"
                    onClick={fillTestCredentials}
                    disabled={isLoading}
                  >
                    <Key className="mr-2 h-3 w-3" />
                    Auto-fill Test Credentials
                  </Button>
                </div>
              </CardContent>
            </Card>
          </CardContent>

          <CardFooter>
            <Button type="submit" className="w-full" disabled={isLoading}>
              {isLoading ? (
                <>
                  <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                  Signing in...
                </>
              ) : (
                <>
                  <LogIn className="mr-2 h-4 w-4" />
                  Sign In
                </>
              )}
            </Button>
          </CardFooter>
        </form>
      </Card>
    </div>
  );
}
