import {
  EditorSettings,
  EditorSettingsContext,
  defaultSettings,
} from '@/contexts/editor-settings-context';
import { usePersistedState } from '@/hooks/use-persisted-state';
import { ReactNode } from 'react';

export const EditorSettingsProvider = ({
  children,
}: {
  children: ReactNode;
}) => {
  const [settings, updateSettings] = usePersistedState<EditorSettings>(
    'editor-settings',
    defaultSettings
  );

  return (
    <EditorSettingsContext.Provider value={{ settings, updateSettings }}>
      {children}
    </EditorSettingsContext.Provider>
  );
};
