import React from 'react';
import { Box, Text, useMantineTheme } from '@mantine/core';

interface LastRunStatusProps {
  status: 'success' | 'running' | 'failed';
  timestamp?: string;
  duration?: string;
}

const statusConfig = {
  success: {
    color: 'green',
    label: 'Success',
    icon: '✓',
  },
  running: {
    color: 'blue',
    label: 'Running',
    icon: '⟳',
  },
  failed: {
    color: 'red',
    label: 'Failed',
    icon: '✕',
  },
};

export const LastRunStatus: React.FC<LastRunStatusProps> = ({
  status,
  timestamp,
  duration,
}) => {
  const theme = useMantineTheme();
  const config = statusConfig[status];
  const statusColor = theme.colors[config.color][6];
  const bgColor = theme.fn.rgba(statusColor, 0.1);

  return (
    <Box
      sx={{
        display: 'flex',
        alignItems: 'center',
        gap: theme.spacing.sm,
        padding: theme.spacing.sm,
        borderRadius: theme.radius.sm,
        backgroundColor: bgColor,
        border: `1px solid ${theme.fn.rgba(statusColor, 0.2)}`,
      }}
    >
      <Text
        sx={{
          color: statusColor,
          fontWeight: 700,
          fontSize: theme.fontSizes.lg,
        }}
      >
        {config.icon}
      </Text>
      <Box>
        <Text
          sx={{
            color: statusColor,
            fontWeight: 600,
            fontSize: theme.fontSizes.sm,
          }}
        >
          {config.label}
        </Text>
        {timestamp && (
          <Text
            sx={{
              color: theme.colorScheme === 'dark' ? theme.colors.gray[4] : theme.colors.gray[6],
              fontSize: theme.fontSizes.xs,
            }}
          >
            {timestamp}
          </Text>
        )}
        {duration && (
          <Text
            sx={{
              color: theme.colorScheme === 'dark' ? theme.colors.gray[5] : theme.colors.gray[7],
              fontSize: theme.fontSizes.xs,
            }}
          >
            Duration: {duration}
          </Text>
        )}
      </Box>
    </Box>
  );
};