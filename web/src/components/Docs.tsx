import { useMemo } from 'react';
import { Docs as PuiDocs, createDocsCatalog, type DocGuide } from '@phiace/puijs';

// Automated ingest of all 67 real Phiano documentation markdown files
const rawDocs = (import.meta as any).glob('../../../docs/**/*.md', {
  query: '?raw',
  eager: true,
  import: 'default',
}) as Record<string, string>;

export interface DocsProps {
  onBackToCockpit: () => void;
  dark: boolean;
  toggleDark: () => void;
}

export function Docs({ onBackToCockpit, dark, toggleDark }: DocsProps) {
  const guides: DocGuide[] = useMemo(() => {
    return createDocsCatalog(rawDocs);
  }, []);

  return (
    <PuiDocs
      guides={guides}
      initialGuideId="02-architecture-overview"
      version="v0.3.1"
      brandTitle="Phiano Docs"
      onBackToApp={onBackToCockpit}
      backToAppLabel="Open Cockpit"
      dark={dark}
      onToggleDark={toggleDark}
    />
  );
}

export default Docs;
