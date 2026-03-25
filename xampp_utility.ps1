<#
.SYNOPSIS
    A beautiful and creative TUI backup script for XAMPP.

.DESCRIPTION
    This script backs up the essential directories of a XAMPP installation
    (htdocs, mysql data, apache conf) into a compressed ZIP archive.
    It features an interactive Text User Interface (TUI) and native progress tracking.
    The interface dynamically centers to the exact width of the terminal.

.NOTES
    Created by: Azeem Ali
    Date: 2026-03-24
#>

$ErrorActionPreference = "Stop"

# ====================================================================
# Global Configuration
# ====================================================================

$global:XamppDir = "C:\xampp"
$global:BackupDest = "D:\xampp_backups"

# Paths (Files or Directories) to include in backup (relative to XamppDir)
$global:IncludePaths = @(
    "htdocs",
    "apache\conf",
    "mysql\data",
    "mysql\bin\my.ini",
    "php\php.ini"
)

# ====================================================================
# Centered TUI Rendering Functions
# ====================================================================

$global:TokyoTheme = @{
    "White"    = "$([char]27)[38;2;192;202;245m"
    "Magenta"  = "$([char]27)[38;2;187;154;247m"
    "Cyan"     = "$([char]27)[38;2;125;207;255m"
    "Blue"     = "$([char]27)[38;2;122;162;247m"
    "Green"    = "$([char]27)[38;2;158;206;106m"
    "Yellow"   = "$([char]27)[38;2;224;175;104m"
    "Red"      = "$([char]27)[38;2;247;118;142m"
    "DarkGray" = "$([char]27)[38;2;86;95;137m"
    "Reset"    = "$([char]27)[0m"
}

Function Write-TokyoOutput {
    param([string]$Text, [string]$ColorName = "White", [switch]$NoNewline)
    $AnsiColor = $global:TokyoTheme[$ColorName]
    if (-not $AnsiColor) { $AnsiColor = $global:TokyoTheme["White"] }
    $Reset = $global:TokyoTheme["Reset"]
    
    if ($NoNewline) {
        Write-Host "${AnsiColor}${Text}${Reset}" -NoNewline
    } else {
        Write-Host "${AnsiColor}${Text}${Reset}"
    }
}

Function Write-Divider {
    param([string]$Color = "Cyan")
    $TermWidth = 80
    if ($Host.UI.RawUI.WindowSize.Width -gt 0) {
        $TermWidth = $Host.UI.RawUI.WindowSize.Width
    }
    
    Write-TokyoOutput ("=" * $TermWidth) $Color
}

Function Write-CenterBlock {
    param([string]$Text, [string]$Color = "White", [int]$BlockWidth = 80, [switch]$NoNewline)
    $TermWidth = 80
    if ($Host.UI.RawUI.WindowSize.Width -gt 0) {
        $TermWidth = $Host.UI.RawUI.WindowSize.Width
    }
    
    $PadCount = [math]::Max(0, [math]::Floor(($TermWidth - $BlockWidth) / 2))
    $Padding = " " * $PadCount
    
    if ($NoNewline) {
        Write-TokyoOutput ($Padding + $Text) $Color -NoNewline
    } else {
        Write-TokyoOutput ($Padding + $Text) $Color
    }
}

Function Write-Banner {
    Clear-Host
    Write-Divider
    Write-CenterBlock " __  __   _      __  __  ____  ____    _   _ _   _ _ _ _         " "Magenta" 65
    Write-CenterBlock " \ \/ /  / \    |  \/  ||  _ \|  _ \  | | | | |_(_) (_) |_ _   _ " "Magenta" 65
    Write-CenterBlock "  \  /  / _ \   | |\/| || |_) | |_) | | | | | __| | | | __| | | |" "Magenta" 65
    Write-CenterBlock "  /  \ / ___ \  | |  | ||  __/|  __/  | |_| | |_| | | | |_| |_| |" "Magenta" 65
    Write-CenterBlock " /_/\_/_/   \_\ |_|  |_||_|   |_|      \___/ \__|_|_|_|\__|\__, |" "Magenta" 65
    Write-CenterBlock "                                                          |___/  " "Magenta" 65
    Write-Divider
    Write-CenterBlock "                 Automated XAMPP Utility Manager                 " "Yellow" 70
    Write-CenterBlock "                       Created by Azeem Ali                      " "Green" 70
    Write-Divider
    Write-Host ""
}

Function Write-Log {
    param([string]$Message, [string]$Level = "INFO", [string]$LogPath)
    $Timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $LogMessage = "[$Timestamp] [$Level] $Message"
    
    switch ($Level) {
        "INFO"    { Write-CenterBlock "  [+] $Message" "Green" 80 }
        "WARNING" { Write-CenterBlock "  [!] $Message" "Yellow" 80 }
        "ERROR"   { Write-CenterBlock "  [X] $Message" "Red" 80 }
        "STEP"    { Write-CenterBlock "  [*] $Message" "Cyan" 80 }
    }
    
    if (-not [string]::IsNullOrEmpty($LogPath)) {
        if (-not (Test-Path -Path (Split-Path $LogPath -Parent))) {
            New-Item -ItemType Directory -Path (Split-Path $LogPath -Parent) -Force | Out-Null
        }
        Add-Content -Path $LogPath -Value $LogMessage -ErrorAction SilentlyContinue
    }
}

Function Write-TerminalProgress {
    param([string]$Activity, [int]$Percent)
    
    $TermWidth = 80
    if ($Host.UI.RawUI.WindowSize.Width -gt 0) {
        $TermWidth = $Host.UI.RawUI.WindowSize.Width
    }
    $BlockWidth = 80
    $PadCount = [math]::Max(0, [math]::Floor(($TermWidth - $BlockWidth) / 2))
    $Padding = " " * $PadCount

    $BarWidth = 20
    $Filled = [math]::Floor(($Percent / 100) * $BarWidth)
    $Empty = $BarWidth - $Filled
    $Bar = ("=" * $Filled) + (" " * $Empty)
    
    $Cyan = $global:TokyoTheme["Cyan"]
    $Reset = $global:TokyoTheme["Reset"]
    Write-Host "`r${Cyan}${Padding}    >> $Activity [$Bar] $Percent%   ${Reset}" -NoNewline
}

Function End-Script {
    Write-Host ""
    Write-CenterBlock "  -> Press any key to return to the main menu..." "Yellow" 80
    $null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
}

# ====================================================================
# Backup Logic
# ====================================================================

Function Invoke-XamppBackup {
    param([switch]$Quiet)
    if (-not $Quiet) { Write-Banner }
    $DateStr = Get-Date -Format "yyyyMMdd_HHmmss"
    $BackupFile = Join-Path -Path $global:BackupDest -ChildPath "xampp_backup_$DateStr.zip"
    $LogFile = Join-Path -Path $global:BackupDest -ChildPath "backup_log_$DateStr.txt"

    if (-not (Test-Path -Path $global:BackupDest)) {
        Write-Log -Message "Creating backup destination folder: $($global:BackupDest)" -Level STEP -LogPath $LogFile
        try {
            New-Item -ItemType Directory -Path $global:BackupDest -Force | Out-Null
        }
        catch {
            Write-Log -Message "Failed to create backup directory. Aborting Backup." -Level ERROR -LogPath ""
            if (-not $Quiet) { End-Script }
            return $null
        }
    }

    Write-Log -Message "Backup Session Started." -Level STEP -LogPath $LogFile
    Write-Log -Message "Target XAMPP Directory : $($global:XamppDir)" -Level INFO -LogPath $LogFile
    Write-Log -Message "Backup Destination     : $($global:BackupDest)" -Level INFO -LogPath $LogFile
    Write-Log -Message "Log File Path          : $LogFile" -Level INFO -LogPath $LogFile
    Write-Host ""

    if (-not (Test-Path -Path $global:XamppDir)) {
        Write-Log -Message "XAMPP directory not found at $($global:XamppDir)! Please re-configure." -Level ERROR -LogPath $LogFile
        if (-not $Quiet) { End-Script }
        return $null
    }

    $TempDir = Join-Path -Path $env:TEMP -ChildPath "xampp_backup_temp_$DateStr"
    New-Item -ItemType Directory -Path $TempDir -Force | Out-Null

    $TotalDirs = $global:IncludePaths.Count
    $CurrentDirIndex = 0

    Write-Log -Message "Stage 1: Gathering files for backup..." -Level STEP -LogPath $LogFile

    foreach ($Item in $global:IncludePaths) {
        $CurrentDirIndex++
        $SourcePath = Join-Path -Path $global:XamppDir -ChildPath $Item
        $DestPath = Join-Path -Path $TempDir -ChildPath $Item

        $Perc = [math]::Floor(($CurrentDirIndex / $TotalDirs) * 100)
        Write-TerminalProgress -Activity "Gathering $Item" -Percent $Perc

        if (Test-Path -Path $SourcePath) {
            $ParentPath = Split-Path -Path $DestPath -Parent
            if (-not (Test-Path -Path $ParentPath)) {
                New-Item -ItemType Directory -Path $ParentPath -Force | Out-Null
            }
            try {
                Copy-Item -Path $SourcePath -Destination $ParentPath -Recurse -Force -ErrorAction SilentlyContinue
                Write-Host ""
                Write-Log -Message "Successfully backed up: $Item" -Level INFO -LogPath $LogFile
            }
            catch {
                Write-Host ""
                Write-Log -Message "Error copying $Item : $_" -Level ERROR -LogPath $LogFile
                Write-TerminalProgress -Activity "Gathering $Item" -Percent $Perc
            }
        }
        else {
            Write-Host ""
            Write-Log -Message "Path not found, skipping: $Item" -Level WARNING -LogPath $LogFile
            Write-TerminalProgress -Activity "Gathering $Item" -Percent $Perc
        }
    }
    Write-Host ""
    Write-Host ""

    Write-Log -Message "Stage 2: Compressing files to ZIP archive..." -Level STEP -LogPath $LogFile
    Write-Log -Message "Archive: $BackupFile" -Level INFO -LogPath $LogFile

    Write-CenterBlock "    >> Zipping files (this may take a few minutes)... " "Cyan" 80 -NoNewline
    try {
        Add-Type -AssemblyName System.IO.Compression.FileSystem
        if (Test-Path $BackupFile) { Remove-Item $BackupFile -Force | Out-Null }
        [System.IO.Compression.ZipFile]::CreateFromDirectory($TempDir, $BackupFile, [System.IO.Compression.CompressionLevel]::Optimal, $false)
        Write-TokyoOutput "[DONE]" "Green"
        Write-Log -Message "Compression completed successfully." -Level INFO -LogPath $LogFile
    }
    catch {
        Write-TokyoOutput "[FAILED]" "Red"
        Write-Log -Message "Failed to compress archive. Error: $_" -Level ERROR -LogPath $LogFile
    }
    Write-Host ""

    Write-Log -Message "Stage 3: Cleaning up temporary files..." -Level STEP -LogPath $LogFile
    try {
        Remove-Item -Path $TempDir -Recurse -Force | Out-Null
        Write-Log -Message "Removed temporary folder: $TempDir" -Level INFO -LogPath $LogFile
    }
    catch {
        Write-Log -Message "Failed to clean up temporary folder: $TempDir" -Level WARNING -LogPath $LogFile
    }
    Write-Host ""

    Write-Log -Message "Backup Session Finished Successfully!" -Level STEP -LogPath $LogFile
    Write-Host ""
    Write-CenterBlock "  =================================================================  " "DarkGray" 80
    
    $SuccessMsg1 = "  SUCCESS: Backup securely stored at: "
    Write-CenterBlock $SuccessMsg1 "Green" 80 -NoNewline
    Write-TokyoOutput $BackupFile "Cyan"
    
    Write-CenterBlock "  =================================================================  " "DarkGray" 80
    Write-Host ""
    
    if (-not $Quiet) {
        Write-Log -Message "Opening backup destination folder..." -Level INFO -LogPath $LogFile
        Invoke-Item -Path $global:BackupDest
        End-Script
    }
    
    return $BackupFile
}

# ====================================================================
# Utility / Management Logic
# ====================================================================

Function Get-XamppReleases {
    return [ordered]@{
        "8.2.12" = "https://sourceforge.net/projects/xampp/files/XAMPP%20Windows/8.2.12/xampp-portable-windows-x64-8.2.12-0-VS16.zip/download"
        "8.1.25" = "https://sourceforge.net/projects/xampp/files/XAMPP%20Windows/8.1.25/xampp-portable-windows-x64-8.1.25-0-VS16.zip/download"
        "8.0.30" = "https://sourceforge.net/projects/xampp/files/XAMPP%20Windows/8.0.30/xampp-portable-windows-x64-8.0.30-0-VS16.zip/download"
        "7.4.33" = "https://sourceforge.net/projects/xampp/files/XAMPP%20Windows/7.4.33/xampp-portable-windows-x64-7.4.33-0-VC15.zip/download"
    }
}

Function Invoke-XamppDownload {
    param([string]$Url, [string]$Dest)
    Write-Log "Downloading XAMPP from SourceForge... This might take a while." -Level STEP
    try {
        Invoke-WebRequest -Uri $Url -OutFile $Dest -UseBasicParsing
        return $true
    } catch {
        Write-Log "Download failed: $_" -Level ERROR
        return $false
    }
}

Function Install-Xampp {
    param([string]$VersionName, [string]$DownloadUrl, [string]$TargetDir)
    
    $TempZip = Join-Path $env:TEMP "xampp_install_$VersionName.zip"
    $TempExtract = Join-Path $env:TEMP "xampp_extract_$VersionName"
    
    if (Test-Path $TempExtract) { Remove-Item $TempExtract -Recurse -Force | Out-Null }
    New-Item -ItemType Directory -Path $TempExtract -Force | Out-Null
    
    Write-Log "Stage: Downloading XAMPP v$VersionName" -Level STEP
    if (-not (Invoke-XamppDownload -Url $DownloadUrl -Dest $TempZip)) { return $false }
    
    Write-Log "Stage: Extracting ZIP archive..." -Level STEP
    Write-CenterBlock "    >> Extracting (this will take several minutes)... " "Cyan" 80 -NoNewline
    try {
        Add-Type -AssemblyName System.IO.Compression.FileSystem
        [System.IO.Compression.ZipFile]::ExtractToDirectory($TempZip, $TempExtract)
        Write-TokyoOutput "[DONE]" "Green"
    } catch {
        Write-TokyoOutput "[FAILED]" "Red"
        Write-Log "Extraction failed: $_" -Level ERROR
        return $false
    }
    
    $ExtractedXamppDir = Join-Path $TempExtract "xampp"
    if (-not (Test-Path $ExtractedXamppDir)) {
        Write-Log "Could not find 'xampp' folder in downloaded zip." -Level ERROR
        return $false
    }
    
    Write-Log "Stage: Moving to $TargetDir ..." -Level STEP
    try {
        if (Test-Path $TargetDir) {
            Write-Log "Target directory $TargetDir already exists. Please remove or rename it first." -Level ERROR
            return $false
        }
        Move-Item -Path $ExtractedXamppDir -Destination $TargetDir -Force
    } catch {
        Write-Log "Failed to move XAMPP to target directory: $_" -Level ERROR
        return $false
    }
    
    Write-Log "Stage: Initializing XAMPP Environment..." -Level STEP
    $SetupBat = Join-Path $TargetDir "setup_xampp.bat"
    if (Test-Path $SetupBat) {
        try {
            $prevDir = Get-Location
            Set-Location $TargetDir
            $null = & $SetupBat
            Set-Location $prevDir
            Write-Log "setup_xampp.bat completed successfully." -Level INFO
        } catch {
            Write-Log "Failed to run setup_xampp.bat : $_" -Level WARNING
        }
    } else {
        Write-Log "setup_xampp.bat not found. Paths may need manual fixing." -Level WARNING
    }
    
    try {
        Remove-Item $TempZip -Force | Out-Null
        Remove-Item $TempExtract -Recurse -Force | Out-Null
    } catch {}
    
    return $true
}

Function Restore-BackupArchive {
    param([string]$BackupZipPath, [string]$TargetXamppDir)
    
    if (-not (Test-Path $BackupZipPath)) {
        Write-Log "Backup file not found: $BackupZipPath" -Level ERROR
        return $false
    }
    if (-not (Test-Path $TargetXamppDir)) {
        Write-Log "XAMPP Directory not found: $TargetXamppDir" -Level ERROR
        return $false
    }
    
    Write-Log "Stage: Restoring backup to $TargetXamppDir" -Level STEP
    
    $DateStr = Get-Date -Format "yyyyMMdd_HHmmss"
    $TempExtract = Join-Path $env:TEMP "xampp_restore_temp_$DateStr"
    New-Item -ItemType Directory -Path $TempExtract -Force | Out-Null
    
    Write-CenterBlock "    >> Extracting backup archive... " "Cyan" 80 -NoNewline
    try {
        Add-Type -AssemblyName System.IO.Compression.FileSystem
        [System.IO.Compression.ZipFile]::ExtractToDirectory($BackupZipPath, $TempExtract)
        Write-TokyoOutput "[DONE]" "Green"
    } catch {
        Write-TokyoOutput "[FAILED]" "Red"
        Write-Log "Failed to extract backup zip: $_" -Level ERROR
        return $false
    }
    
    Write-Log "Copying contents to $TargetXamppDir ..." -Level INFO
    
    foreach ($Item in $global:IncludePaths) {
        $ExtractedPath = Join-Path $TempExtract $Item
        $DestPath = Join-Path $TargetXamppDir $Item
        
        if (Test-Path $ExtractedPath) {
            try {
                $ParentPath = Split-Path $DestPath -Parent
                if (-not (Test-Path $ParentPath)) { New-Item -ItemType Directory -Path $ParentPath -Force | Out-Null }
                
                if ((Get-Item $ExtractedPath) -is [System.IO.FileInfo]) {
                    Copy-Item -Path $ExtractedPath -Destination $DestPath -Force
                } else {
                    Copy-Item -Path "$ExtractedPath\*" -Destination $DestPath -Recurse -Force
                }
                Write-Log "Restored: $Item" -Level INFO
            } catch {
                Write-Log "Failed to restore $Item : $_" -Level WARNING
            }
        }
    }
    
    try { Remove-Item $TempExtract -Recurse -Force | Out-Null } catch {}
    Write-Log "Restore Completed Successfully." -Level STEP
    return $true
}

Function Select-XamppVersion {
    $Releases = Get-XamppReleases
    $Keys = @($Releases.Keys)
    Write-Host ""
    Write-CenterBlock "Available XAMPP (PHP) Versions:" "White" 80
    Write-Host ""
    for ($i = 0; $i -lt $Keys.Count; $i++) {
        Write-CenterBlock "[$($i + 1)] $($Keys[$i])" "Cyan" 80
    }
    Write-Host ""
    Write-CenterBlock "Select a version (1-$($Keys.Count)): " "Yellow" 60 -NoNewline
    $choiceStr = Read-Host
    if ([int]::TryParse($choiceStr, [ref]$null)) {
        $idx = [int]$choiceStr - 1
        if ($idx -lt 0 -or $idx -ge $Keys.Count) { return $null }
        return @{ Name = $Keys[$idx]; Url = $Releases[$Keys[$idx]] }
    }
    return $null
}

Function Invoke-XamppUpgrade {
    param([string]$ModeName)
    Write-Banner
    Write-CenterBlock "--- $ModeName XAMPP ---" "White" 80
    
    $VersionInfo = Select-XamppVersion
    if (-not $VersionInfo) {
        Write-CenterBlock "Invalid selection. Aborting." "Red" 80
        End-Script
        return
    }
    
    Write-Host ""
    Write-CenterBlock "Starting $ModeName to version $($VersionInfo.Name)..." "Magenta" 80
    Start-Sleep -Seconds 2
    
    if (-not (Test-Path $global:XamppDir)) {
        Write-Log "Current XAMPP installation not found at $($global:XamppDir). Cannot $ModeName." -Level ERROR
        End-Script
        return
    }
    
    Write-Log "Creating automatic pre-$ModeName backup..." -Level STEP
    $BackupZip = Invoke-XamppBackup -Quiet
    if (-not $BackupZip -or -not (Test-Path $BackupZip)) {
        Write-Log "Backup failed! Aborting $ModeName safely." -Level ERROR
        End-Script
        return
    }
    
    $DateStr = Get-Date -Format "yyyyMMdd_HHmmss"
    $OldDir = "$($global:XamppDir)_old_$DateStr"
    Write-Log "Renaming current XAMPP to: $OldDir" -Level STEP
    try {
        Rename-Item -Path $global:XamppDir -NewName (Split-Path $OldDir -Leaf) -Force
    } catch {
        Write-Log "Failed to rename current XAMPP dir. Aborting." -Level ERROR
        End-Script
        return
    }
    
    if (-not (Install-Xampp -VersionName $VersionInfo.Name -DownloadUrl $VersionInfo.Url -TargetDir $global:XamppDir)) {
        Write-Log "Installation Failed! You can manually revert by renaming $OldDir back to xampp." -Level ERROR
        End-Script
        return
    }
    
    if (-not (Restore-BackupArchive -BackupZipPath $BackupZip -TargetXamppDir $global:XamppDir)) {
        Write-Log "Restoration partially failed. Your data is backed up at: $BackupZip" -Level WARNING
    }
    
    Write-Host ""
    Write-CenterBlock "  =================================================================  " "DarkGray" 80
    Write-CenterBlock "  $ModeName Completed Successfully!  " "Green" 80
    Write-CenterBlock "  Old version backed up completely to: $OldDir  " "DarkGray" 80
    Write-CenterBlock "  =================================================================  " "DarkGray" 80
    End-Script
}

Function Invoke-XamppInstall {
    Write-Banner
    Write-CenterBlock "--- INSTALL XAMPP ---" "White" 80
    
    if (Test-Path $global:XamppDir) {
        Write-CenterBlock "XAMPP is already installed at $($global:XamppDir)!" "Red" 80
        Write-CenterBlock "Please use Upgrade/Downgrade or Reinstall instead." "Yellow" 80
        End-Script
        return
    }
    
    $VersionInfo = Select-XamppVersion
    if (-not $VersionInfo) {
        Write-CenterBlock "Invalid selection. Aborting." "Red" 80
        End-Script
        return
    }
    
    if (Install-Xampp -VersionName $VersionInfo.Name -DownloadUrl $VersionInfo.Url -TargetDir $global:XamppDir) {
        Write-Host ""
        Write-CenterBlock "  =================================================================  " "DarkGray" 80
        Write-CenterBlock "  Install Completed Successfully!  " "Green" 80
        Write-CenterBlock "  =================================================================  " "DarkGray" 80
    }
    End-Script
}

Function Invoke-XamppRestoreMenu {
    Write-Banner
    Write-CenterBlock "--- RESTORE BACKUP ---" "White" 80
    
    if (-not (Test-Path $global:BackupDest)) {
        Write-CenterBlock "Backup destination folder not found." "Red" 80
        End-Script
        return
    }
    
    $Zips = Get-ChildItem -Path $global:BackupDest -Filter "*.zip" | Sort-Object CreationTime -Descending
    if ($Zips.Count -eq 0) {
        Write-CenterBlock "No backups found in $($global:BackupDest)." "Red" 80
        End-Script
        return
    }
    
    Write-Host ""
    for ($i = 0; $i -lt $Zips.Count; $i++) {
        Write-CenterBlock "[$($i + 1)] $($Zips[$i].Name) ($([math]::Round($Zips[$i].Length/1MB, 2)) MB)" "Cyan" 80
    }
    Write-Host ""
    Write-CenterBlock "Select a backup to restore (1-$($Zips.Count)): " "Yellow" 60 -NoNewline
    $choiceStr = Read-Host
    if ([int]::TryParse($choiceStr, [ref]$null)) {
        $idx = [int]$choiceStr - 1
        if ($idx -ge 0 -and $idx -lt $Zips.Count) {
            $SelectedZip = $Zips[$idx].FullName
            
            Write-Host ""
            Write-CenterBlock "WARNING: This will overwrite files in $($global:XamppDir)!" "Red" 80
            Write-CenterBlock "Proceed? (y/N): " "Yellow" 60 -NoNewline
            $confirm = Read-Host
            if ($confirm -match "^y" -or $confirm -match "^Y") {
                if (Restore-BackupArchive -BackupZipPath $SelectedZip -TargetXamppDir $global:XamppDir) {
                    Write-Host ""
                    Write-CenterBlock "Restore completely finished!" "Green" 80
                }
            } else {
                Write-CenterBlock "Restore cancelled." "DarkGray" 80
            }
        } else {
            Write-CenterBlock "Invalid choice." "Red" 80
        }
    }
    End-Script
}

Function Invoke-CheckXamppUpdate {
    Write-Banner
    Write-CenterBlock "--- CHECK FOR UPDATES ---" "White" 80
    Write-Host ""
    
    $CurrentVersion = "Unknown"
    $PhpExe = Join-Path $global:XamppDir "php\php.exe"
    if (Test-Path $PhpExe) {
        try {
            $PhpVerOutput = & $PhpExe -v | Select-Object -First 1
            if ($PhpVerOutput -match "PHP\s+([\d\.]+)") {
                $CurrentVersion = $matches[1]
            }
        } catch {}
    }
    
    if ($CurrentVersion -eq "Unknown") {
        Write-CenterBlock "Could not determine local XAMPP version." "Yellow" 80
        Write-CenterBlock "Make sure XAMPP is installed at $($global:XamppDir)" "DarkGray" 80
    } else {
        Write-CenterBlock "Current XAMPP (PHP) Version: $CurrentVersion" "Cyan" 80
    }
    
    $Releases = Get-XamppReleases
    $LatestVersion = @($Releases.Keys)[0]
    
    Write-Host ""
    Write-CenterBlock "Latest Available Version: $LatestVersion" "Green" 80
    Write-Host ""
    
    if ($CurrentVersion -eq $LatestVersion) {
        Write-CenterBlock "You are fully up to date!" "Magenta" 80
    } elseif ($CurrentVersion -ne "Unknown") {
        try {
            if ([version]$CurrentVersion -lt [version]$LatestVersion) {
                Write-CenterBlock "An update is available! Go to 'Upgrade/Update XAMPP' to install." "Yellow" 80
            } else {
                Write-CenterBlock "You are fully up to date!" "Magenta" 80
            }
        } catch {
            Write-CenterBlock "Version comparison failed." "DarkGray" 80
        }
    }
    
    End-Script
}

# ====================================================================
# Main TUI Menu Loop
# ====================================================================

while ($true) {
    Write-Banner
    
    Write-CenterBlock "--- MAIN MENU ---" "White" 80
    Write-Host ""
    Write-CenterBlock "[1] Install Fresh XAMPP" "Green" 80
    Write-CenterBlock "[2] Upgrade/Update XAMPP" "Green" 80
    Write-CenterBlock "[3] Downgrade/Reinstall XAMPP" "Green" 80
    Write-CenterBlock "[4] Backup Current Setup" "Yellow" 80
    Write-CenterBlock "[5] Restore from Backup" "Yellow" 80
    Write-CenterBlock "[6] Check for XAMPP Updates" "Magenta" 80
    Write-Host ""
    
    $configXamppStr = "[7] Config XAMPP Path     (Current: $($global:XamppDir))"
    Write-CenterBlock $configXamppStr "Cyan" 80
    
    $configDestStr =  "[8] Config Backup Dest    (Current: $($global:BackupDest))"
    Write-CenterBlock $configDestStr "Cyan" 80
    
    Write-Host ""
    Write-CenterBlock "[9] Exit" "Red" 80
    Write-Host ""
    Write-Divider
    Write-CenterBlock "Please select an option (1-9): " "Yellow" 60 -NoNewline

    $choiceStr = Read-Host

    switch ($choiceStr) {
        "1" { Invoke-XamppInstall }
        "2" { Invoke-XamppUpgrade -ModeName "Upgrade" }
        "3" { Invoke-XamppUpgrade -ModeName "Downgrade" }
        "4" { Invoke-XamppBackup }
        "5" { Invoke-XamppRestoreMenu }
        "6" { Invoke-CheckXamppUpdate }
        "7" {
            Write-Host "`n`n"
            Write-CenterBlock "[?] Enter new XAMPP directory path: " "Yellow" 60 -NoNewline
            $newDir = Read-Host
            if (-not [string]::IsNullOrWhiteSpace($newDir)) {
                $global:XamppDir = $newDir
            }
        }
        "8" {
            Write-Host "`n`n"
            Write-CenterBlock "[?] Enter new Backup Destination path: " "Yellow" 60 -NoNewline
            $newDest = Read-Host
            if (-not [string]::IsNullOrWhiteSpace($newDest)) {
                $global:BackupDest = $newDest
            }
        }
        "9" {
            Write-Host "`n`n"
            Write-CenterBlock "Exiting... Thank you for using XAMPP Utility Manager." "DarkGray" 80
            Start-Sleep -Seconds 1
            exit
        }
    }
}
