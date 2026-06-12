import React from 'react';
import { Badge } from '@/components/ui/badge';
import { useTr } from '@/hooks/useTr';

interface ClassifierPropertiesPanelProps {
  // ... existing props
}

export const ClassifierPropertiesPanel: React.FC<ClassifierPropertiesPanelProps> = (props) => {
  const { tr } = useTr();

  // ... existing code

  return (
    <div>
      {/* ... existing JSX */}
      {category.category_id === 'others_category' ? (
        <Badge variant="secondary">{tr('classifier.othersBadge', 'Others')}</Badge>
      ) : (
        // ... existing editable row
      )}
      {/* ... existing JSX */}
    </div>
  );
};