import {
  EditorSettings,
  EditorSettingsContext,
  defaultSettings,
  deserializeSettings,
} from '@/contexts/editor-settings-context';
import { usePersistedState } from '@/hooks/use-persisted-state';
import { ReactNode } from 'react';

const persistenceOptions = { deserialize: deserializeSettings };

export const EditorSettingsProvider = ({
  children,
}: {
  children: ReactNode;
}) => {
  const [settings, setSettings] = usePersistedState<EditorSettings>(
    'editor-settings',
    defaultSettings,
    persistenceOptions
  );

  const updateSettings = (newSettings: Partial<EditorSettings>) => {
    setSettings((prevSettings) => ({ ...prevSettings, ...newSettings }));
  };

  return (
    <EditorSettingsContext.Provider value={{ settings, updateSettings }}>
      {children}
    </EditorSettingsContext.Provider>
  );
};
