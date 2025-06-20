// src/app/layout.tsx
"use client";

import { Inter } from "next/font/google";
import "@/styles/globals.css";
import { ModalRenderer } from "@/components/ModalRenderer";

const inter = Inter({ 
  subsets: ["latin"],
  variable: "--font-sans",
});

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" className={inter.variable}>
      <body className="font-sans antialiased">
        {children}
        <ModalRenderer />
      </body>
    </html>
  );
}
