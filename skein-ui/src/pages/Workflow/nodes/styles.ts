export const handleStyle = `
  .skein-handle-container {
    position: absolute;
  }
  .skein-handle-plus {
    opacity: 0;
    pointer-events: none;
    position: absolute;
    left: 10px;
    top: 50%;
    transform: translateY(-50%);
    z-index: 1000;
    transition: opacity 0.15s ease;
  }
  .skein-handle-container:hover .skein-handle-plus,
  .skein-handle-plus:hover {
    opacity: 1;
    pointer-events: all;
  }
  .skein-workflow-node {
    width: 240px;
    border-radius: 14px;
    border: 1px solid var(--skein-border-dim);
    background: var(--skein-bg-base);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.02), 0 1px 2px rgba(0, 0, 0, 0.01);
    display: flex;
    flex-direction: column;
    position: relative;
    cursor: pointer;
    transition: all 0.25s cubic-bezier(0.25, 0.8, 0.25, 1);
  }
  .skein-workflow-node:hover {
    transform: translateY(-2px);
    border-color: var(--skein-border-base);
    box-shadow: 0 8px 18px rgba(0, 0, 0, 0.04), 0 2px 4px rgba(0, 0, 0, 0.02);
  }
  .skein-workflow-node.selected {
    border: 1px solid var(--skein-accent) !important;
    box-shadow: 0 0 0 3px rgba(21, 90, 239, 0.15), 0 12px 24px rgba(21, 90, 239, 0.1), 0 4px 12px rgba(0, 0, 0, 0.04) !important;
  }
  .skein-workflow-node.selected:hover {
    transform: translateY(-2px);
    box-shadow: 0 0 0 3px rgba(21, 90, 239, 0.2), 0 16px 32px rgba(21, 90, 239, 0.14), 0 6px 16px rgba(0, 0, 0, 0.06) !important;
  }
  .skein-node-icon-container {
    width: 24px;
    height: 24px;
    border-radius: 7px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    transition: all 0.2s ease;
  }
`;

