# Variables Globales CSS (Estilo Institutional Dark)

```css
:root {
  /* Fondo general y layout (Estilo Navy Dark) */
  --color-bg-base: #0E1226; /* Fondo principal más oscuro */
  --color-bg-secondary: #0F121E; /* Fondo secundario profundo */
  --color-bg-card: #151826; /* Fondo de tarjetas */
  --color-bg-card-soft: #1A1D2D;
  --color-bg-terminal: #0A0C11;

  /* Bordes (sutiles y sin líneas agresivas para mantener el glassmorphism) */
  --color-border: rgba(255, 255, 255, 0.04);
  
  /* Textos */
  --color-text-primary: #E2E8F0;
  --color-text-secondary: #94A3B8;
  --color-text-muted: #64748B;
  
  /* Acentos y Branding */
  --color-primary: #3B82F6;
  --color-primary-light: #9bc3f3;
  --color-primary-dark: #2563EB;
  --gradient-primary: linear-gradient(135deg, #3B82F6 0%, #2563EB 100%);
  
  /* Estados / Semántica (Claves para P&L y Logs) */
  --color-success: #10B981; /* Verde vibrante minimalista */
  --color-danger: #EF4444; /* Rojo vibrante */
  --color-warning: #F59E0B;
  
  /* Tipografía base */
  --font-sans: 'Inter', system-ui, -apple-system, sans-serif;
  --font-mono: 'JetBrains Mono', monospace; /* Ideal para números de bolsa */
  
  /* Radios de borde consistentes */
  --radius-sm: 6px;
  --radius-md: 8px;
  --radius-lg: 12px;
}
```

## Sugerencias de uso en Vue 3
Para el nuevo Frontend, seguiremos inyectando estas variables globalmente. Te recomiendo usarlas junto con efectos de `backdrop-filter: blur(10px)` para lograr un *Glassmorphism* elegante en los modales de configuración que nos pide el jurado.
