// src/types/commands.ts
export interface TauriCommands {
  greet: {
    args: void;
    result: string;
  };
  analyze_system: {
    args: void;
    result: string;
  };
}
