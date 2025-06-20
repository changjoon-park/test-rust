// src/components/modals/DemoModal.tsx
export default function DemoModal({ id }: { id?: string }) {
  return (
    <div className="space-y-4">
      <p className="text-sm text-muted-foreground">
        This is a demo modal using the Modal Route pattern.
        {id && ` Showing content for ID: ${id}`}
      </p>
      <div className="rounded-lg bg-muted p-4">
        <h4 className="font-semibold mb-2">Key Features:</h4>
        <ul className="text-sm space-y-1">
          <li>• URL-based state management</li>
          <li>• Browser back button support</li>
          <li>• Shareable links</li>
          <li>• Survives page refresh</li>
        </ul>
      </div>
    </div>
  );
}
