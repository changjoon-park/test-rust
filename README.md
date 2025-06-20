# Tauri + Next.js Desktop App

A modern desktop application built with Tauri v2, Next.js 15, and shadcn/ui.

## 🚀 Tech Stack

- **[Tauri v2](https://tauri.app/)** - Build smaller, faster, and more secure desktop applications
- **[Next.js 15](https://nextjs.org/)** - React framework with static export
- **[Tailwind CSS v4](https://tailwindcss.com/)** - Utility-first CSS framework
- **[shadcn/ui](https://ui.shadcn.com/)** - Re-usable components built with Radix UI and Tailwind
- **[TypeScript](https://www.typescriptlang.org/)** - Type safety
- **[Rust](https://www.rust-lang.org/)** - Backend language for Tauri

## 📋 Prerequisites

- **Node.js** >= 18.0.0
- **pnpm** >= 8.0.0
- **Rust** >= 1.70.0
- **Platform-specific requirements:**
  - Windows: [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
  - macOS: Xcode Command Line Tools
  - Linux: `build-essential`, `libwebkit2gtk-4.1-dev`, `libgtk-3-dev`

## 🛠️ Installation

1. Clone the repository:
```bash
git clone <your-repo-url>
cd monori
```

2. Install dependencies:
```bash
pnpm install
```

3. Run development server:
```bash
pnpm tauri dev
```

## 📦 Available Scripts

```bash
# Development
pnpm tauri dev        # Run app in development mode
pnpm dev              # Run Next.js dev server only

# Build
pnpm tauri build      # Build for production
pnpm build            # Build Next.js only

# Code Quality
pnpm lint             # Run ESLint and Biome
pnpm check-types      # TypeScript type checking
```

## 🏗️ Project Structure

```
monori/
├── src/                    # Next.js/React source
│   ├── app/               # App router pages
│   ├── components/        # React components
│   │   └── ui/           # shadcn/ui components
│   ├── hooks/            # Custom React hooks
│   ├── lib/              # Utility functions
│   └── styles/           # Global styles
├── src-tauri/            # Rust backend
│   ├── src/              # Rust source code
│   ├── Cargo.toml        # Rust dependencies
│   └── tauri.conf.json   # Tauri configuration
├── public/               # Static assets
└── package.json          # Node dependencies
```

## 🎨 Adding UI Components

This project uses shadcn/ui. To add new components:

```bash
# Add a new component
npx shadcn@latest add button

# Add multiple components
npx shadcn@latest add card badge separator

# View all available components
npx shadcn@latest add
```

## 🔧 Configuration

### Tauri Configuration
Edit `src-tauri/tauri.conf.json` to configure:
- Window settings (size, title, etc.)
- App permissions
- Build settings

### Next.js Configuration
Edit `next.config.ts` for:
- Static export settings
- Image optimization
- Build output

## 📱 Building for Production

### Desktop App

```bash
# Build for current platform
pnpm tauri build

# Output locations:
# Windows: src-tauri/target/release/[app-name].exe
# macOS: src-tauri/target/release/bundle/dmg/[app-name].dmg
# Linux: src-tauri/target/release/bundle/appimage/[app-name].AppImage
```

### Platform-specific builds

```bash
# Windows only
pnpm tauri build --target x86_64-pc-windows-msvc

# macOS only
pnpm tauri build --target x86_64-apple-darwin
pnpm tauri build --target aarch64-apple-darwin

# Linux only
pnpm tauri build --target x86_64-unknown-linux-gnu
```

## 🐛 Troubleshooting

### Common Issues

1. **Build fails with "Module not found"**
   ```bash
   # Clear cache and reinstall
   rm -rf node_modules pnpm-lock.yaml
   pnpm install
   ```

2. **Tauri command not found**
   ```bash
   # Ensure Rust is properly installed
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. **White screen on startup**
   - Check console for errors (Ctrl+Shift+I or Cmd+Option+I)
   - Ensure all dependencies are installed
   - Try rebuilding: `pnpm tauri build --debug`

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Tauri](https://tauri.app/) for the amazing framework
- [shadcn/ui](https://ui.shadcn.com/) for beautiful components
- [Next.js](https://nextjs.org/) for the React framework
