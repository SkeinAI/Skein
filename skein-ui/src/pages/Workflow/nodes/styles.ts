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
`;
