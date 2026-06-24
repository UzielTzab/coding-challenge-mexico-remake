export const useFormatters = () => {
  const formatUSD = (value: number | string) => {
    const num = typeof value === 'string' ? parseFloat(value) : value;
    return new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD' }).format(num || 0);
  };
  
  const formatBTC = (value: number | string) => {
    const num = typeof value === 'string' ? parseFloat(value) : value;
    return (num || 0).toFixed(8) + ' BTC';
  };
  
  const formatPercent = (value: number | string) => {
    const num = typeof value === 'string' ? parseFloat(value) : value;
    return (num || 0).toFixed(2) + '%';
  };

  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleString();
  };

  return { formatUSD, formatBTC, formatPercent, formatDate };
};
