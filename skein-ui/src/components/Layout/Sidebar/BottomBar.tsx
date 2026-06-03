import { Box, Group, Tooltip, ActionIcon } from '@mantine/core';
import { IconSettings } from '@tabler/icons-react';
import { useTranslation } from 'react-i18next';
import { useUiStore } from '@/store/uiStore';
import { ThemeToggle } from '@/components/Common/ThemeToggle';

export function BottomBar() {
  const { t } = useTranslation();
  const { setSettingsOpen } = useUiStore();

  return (
    <Box style={{ padding: '12px 16px', borderTop: '1px solid var(--skein-border-dim)' }}>
      <Group align="center" gap="xs" wrap="nowrap">
        {/* 设置按钮 */}
        <Tooltip label={t('common.settings')} withArrow>
          <ActionIcon
            variant="subtle"
            color="gray"
            size="sm"
            onClick={() => setSettingsOpen(true)}
            aria-label="Settings"
          >
            <IconSettings size={16} />
          </ActionIcon>
        </Tooltip>

        {/* 主题切换按钮 */}
        <ThemeToggle />
      </Group>
    </Box>
  );
}
