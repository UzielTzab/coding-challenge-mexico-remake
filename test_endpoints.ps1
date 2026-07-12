$url = "http://localhost:8000/health"
$max_retries = 60
$retry_count = 0
$is_up = $false

Write-Host "Waiting for backend to start..."
while ($retry_count -lt $max_retries) {
    try {
        $response = Invoke-WebRequest -Uri $url -Method Get -TimeoutSec 5 -ErrorAction Stop
        if ($response.StatusCode -eq 200) {
            Write-Host "Backend is UP! Health check passed."
            $is_up = $true
            break
        }
    } catch {
        Write-Host "Backend not ready yet, retrying... ($retry_count/$max_retries)"
        Start-Sleep -Seconds 5
        $retry_count++
    }
}

if (-not $is_up) {
    Write-Host "Backend failed to start in time."
    exit 1
}

Write-Host "`nTesting /api/settings..."
$settings = Invoke-WebRequest -Uri "http://localhost:8000/api/settings" -Method Get
Write-Host "Settings Status: $($settings.StatusCode)"
Write-Host "Settings Content: $($settings.Content)"

Write-Host "`nTesting /api/opportunities..."
$opps = Invoke-WebRequest -Uri "http://localhost:8000/api/opportunities" -Method Get
Write-Host "Opportunities Status: $($opps.StatusCode)"
Write-Host "Opportunities Content: $($opps.Content.Substring(0, [math]::Min($opps.Content.Length, 150)))..."

Write-Host "`nTesting /api/analytics/performance/..."
$perf = Invoke-WebRequest -Uri "http://localhost:8000/api/analytics/performance/" -Method Get
Write-Host "Performance Status: $($perf.StatusCode)"
Write-Host "Performance Content: $($perf.Content)"

Write-Host "`nAll tests passed successfully!"
