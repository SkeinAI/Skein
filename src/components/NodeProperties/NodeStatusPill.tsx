import React from 'react';
import { useTranslation } from 'react-i18next';

interface NodeStatusPillProps {
  status: 'done' | 'running' | 'failed' | string;
}

const NodeStatusPill: React.FC<NodeStatusPillProps> = ({ status }) => {
  const { t } = useTranslation();

  const statusConfig: Record<string, { labelKey: string; colorClass: string }> = {
    done: {
      labelKey: 'debug.status.success',
      colorClass: 'bg-green-100 text-green-800',
    },
    running: {
      labelKey: 'debug.status.running',
      colorClass: 'bg-blue-100 text-blue-800',
    },
    failed: {
      labelKey: 'debug.status.failed',
      colorClass: 'bg-red-100 text-red-800',
    },
  };

  const config = statusConfig[status] || {
    labelKey: 'debug.status.unknown',
    colorClass: 'bg-gray-100 text-gray-800',
  };

  return (
    <span
      className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${config.colorClass}`}
    >
      {t(config.labelKey)}
    </span>
  );
};

export default NodeStatusPill;