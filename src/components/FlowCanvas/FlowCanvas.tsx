import { ActionIcon, Tooltip } from '@mantine/core';
import { useTranslation } from 'react-i18next';
import { IconArrowLeft, IconVariable, IconBug } from '@tabler/icons-react';

// ... existing imports and component code ...

export function FlowCanvas() {
  const { t } = useTranslation();

  // ... existing state and handlers ...

  return (
    <div className="flow-canvas">
      <div className="toolbar">
        <Tooltip label={t('toolbar.back')}>
          <ActionIcon
            variant="subtle"
            onClick={handleBack}
            aria-label={t('toolbar.back')}
          >
            <IconArrowLeft size={18} />
          </ActionIcon>
        </Tooltip>

        <Tooltip label={t('toolbar.environmentVariables')}>
          <ActionIcon
            variant="subtle"
            onClick={toggleEnvironmentVariables}
            aria-label={t('toolbar.environmentVariables')}
          >
            <IconVariable size={18} />
          </ActionIcon>
        </Tooltip>

        <Tooltip label={t('toolbar.debugRun')}>
          <ActionIcon
            variant="subtle"
            onClick={toggleDebugRun}
            aria-label={t('toolbar.debugRun')}
          >
            <IconBug size={18} />
          </ActionIcon>
        </Tooltip>
      </div>
      {/* ... rest of the component ... */}
    </div>
  );
}