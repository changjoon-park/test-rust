// src/components/modals/SettingsModal.tsx
export default function SettingsModal() {
  return (
    <div className="space-y-4">
      <p className="text-sm text-muted-foreground">
        Settings modal example. This could contain app preferences, user settings, etc.
      </p>
      <div className="space-y-2">
        <label className="text-sm font-medium">Theme</label>
        <select className="w-full p-2 rounded border">
          <option>Light</option>
          <option>Dark</option>
          <option>System</option>
        </select>
      </div>
    </div>
  );
}
