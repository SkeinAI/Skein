import { Box, Group, Text } from '@mantine/core';

export function SkeinLogo() {
  return (
    <Group px="md" py="sm" gap="sm" mb="xs">
      <Box
        style={{
          width: 28,
          height: 28,
          borderRadius: 8,
          background: 'var(--skein-accent)',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          flexShrink: 0,
          boxShadow: '0 2px 8px rgba(21, 90, 239, 0.3)',
        }}
      >
        {/* Skein mark: two intertwined loops of thread */}
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none">
          <ellipse
            cx="12"
            cy="12"
            rx="3.4"
            ry="7.6"
            transform="rotate(-35 12 12)"
            stroke="white"
            strokeWidth="1.7"
          />
          <ellipse
            cx="12"
            cy="12"
            rx="3.4"
            ry="7.6"
            transform="rotate(35 12 12)"
            stroke="white"
            strokeWidth="1.7"
          />
          <circle cx="12" cy="12" r="1.5" fill="white" />
        </svg>
      </Box>
      <Text fw={700} size="lg" style={{ letterSpacing: '0.5px' }}>Skein</Text>
    </Group>
  );
}
