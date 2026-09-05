import { createContext, useContext } from 'react';

export interface EditorSettings {
  fontSize: number;
  keybindings: 'default' | 'vim';
  lineNumbers: boolean;
  lineWrapping: boolean;
  tabSize: number;
}

export const defaultSettings: EditorSettings = {
  fontSize: 14,
  keybindings: 'default',
  lineNumbers: true,
  lineWrapping: true,
  tabSize: 2,
};

export function deserializeSettings(value: string): EditorSettings {
  const saved: unknown = JSON.parse(value);

  if (saved === null || typeof saved !== 'object' || Array.isArray(saved)) {
    return defaultSettings;
  }

  const settings = saved as Record<string, unknown>;

  return {
    fontSize:
      typeof settings.fontSize === 'number' &&
      [12, 14, 16, 18].includes(settings.fontSize)
        ? settings.fontSize
        : defaultSettings.fontSize,
    keybindings:
      settings.keybindings === 'default' || settings.keybindings === 'vim'
        ? settings.keybindings
        : defaultSettings.keybindings,
    lineNumbers:
      typeof settings.lineNumbers === 'boolean'
        ? settings.lineNumbers
        : defaultSettings.lineNumbers,
    lineWrapping:
      typeof settings.lineWrapping === 'boolean'
        ? settings.lineWrapping
        : defaultSettings.lineWrapping,
    tabSize:
      typeof settings.tabSize === 'number' &&
      [2, 4, 8].includes(settings.tabSize)
        ? settings.tabSize
        : defaultSettings.tabSize,
  };
}

type EditorSettingsContextType = {
  settings: EditorSettings;
  updateSettings: (settings: Partial<EditorSettings>) => void;
};

export const EditorSettingsContext = createContext<
  EditorSettingsContextType | undefined
>(undefined);

export const useEditorSettings = () => {
  const context = useContext(EditorSettingsContext);

  if (context === undefined) {
    throw new Error(
      'useEditorSettings must be used within an EditorSettingsProvider'
    );
  }

  return context;
};
