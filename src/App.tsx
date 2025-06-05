import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import "./App.css";

interface CheckResult {
  분류: string;
  항목코드: string;
  점검항목: string;
  중요도: "상" | "중" | "하";
  점검결과: "양호" | "취약" | "점검 실패" | "수동 점검";
  점검내용: string;
}

interface SecurityReport {
  ComputerName: string;
  DateTime: string;
  OS: string;
  Version: string;
  Results: CheckResult[];
}

interface ProgressPayload {
  current: number;
  total: number;
  message: string;
  current_check?: CheckResult;
}

function App() {
  const [isChecking, setIsChecking] = useState(false);
  const [progress, setProgress] = useState<ProgressPayload | null>(null);
  const [report, setReport] = useState<SecurityReport | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    // Listen for progress updates
    const unlisten = listen<ProgressPayload>("check-progress", (event) => {
      setProgress(event.payload);
    });

    return () => {
      unlisten.then(fn => fn());
    };
  }, []);

  const runSecurityCheck = async () => {
    setIsChecking(true);
    setError(null);
    setReport(null);
    setProgress(null);

    try {
      const result = await invoke<SecurityReport>("run_security_check");
      setReport(result);
    } catch (err) {
      setError(err as string);
    } finally {
      setIsChecking(false);
      setProgress(null);
    }
  };

  const getStatusColor = (status: CheckResult["점검결과"]) => {
    switch (status) {
      case "양호": return "#4CAF50";
      case "취약": return "#F44336";
      case "점검 실패": return "#FF9800";
      case "수동 점검": return "#2196F3";
      default: return "#757575";
    }
  };

  const getImportanceColor = (importance: CheckResult["중요도"]) => {
    switch (importance) {
      case "상": return "#F44336";
      case "중": return "#2196F3";
      case "하": return "#9E9E9E";
      default: return "#757575";
    }
  };

  return (
    <main className="container">
      <h1>Windows 보안 점검 도구</h1>
      
      <div className="check-section">
        <button 
          onClick={runSecurityCheck} 
          disabled={isChecking}
          className="check-button"
        >
          {isChecking ? "점검 중..." : "보안 점검 시작"}
        </button>

        {/* Progress Section */}
        {progress && (
          <div className="progress-section">
            <div className="progress-bar">
              <div 
                className="progress-fill"
                style={{ width: `${(progress.current / progress.total) * 100}%` }}
              />
            </div>
            <p className="progress-text">
              {progress.message} ({progress.current}/{progress.total})
            </p>
          </div>
        )}

        {/* Error Section */}
        {error && (
          <div className="error-section">
            <h3>오류 발생</h3>
            <p>{error}</p>
          </div>
        )}

        {/* Results Section */}
        {report && (
          <div className="results-section">
            <div className="report-header">
              <h2>보안 점검 결과</h2>
              <div className="report-info">
                <p>컴퓨터: {report.ComputerName}</p>
                <p>점검 시간: {report.DateTime}</p>
                <p>OS: {report.OS}</p>
              </div>
            </div>

            <div className="results-summary">
              <div className="summary-item">
                <span>총 점검 항목:</span>
                <strong>{report.Results.length}</strong>
              </div>
              <div className="summary-item">
                <span>양호:</span>
                <strong style={{ color: "#4CAF50" }}>
                  {report.Results.filter(r => r.점검결과 === "양호").length}
                </strong>
              </div>
              <div className="summary-item">
                <span>취약:</span>
                <strong style={{ color: "#F44336" }}>
                  {report.Results.filter(r => r.점검결과 === "취약").length}
                </strong>
              </div>
            </div>

            <div className="results-list">
              {report.Results.map((result, index) => (
                <div key={index} className="result-item">
                  <div className="result-header">
                    <span className="result-code">{result.항목코드}</span>
                    <span className="result-category">{result.분류}</span>
                    <span 
                      className="result-importance"
                      style={{ color: getImportanceColor(result.중요도) }}
                    >
                      중요도: {result.중요도}
                    </span>
                  </div>
                  <h4 className="result-title">{result.점검항목}</h4>
                  <div className="result-status">
                    <span 
                      className="status-badge"
                      style={{ backgroundColor: getStatusColor(result.점검결과) }}
                    >
                      {result.점검결과}
                    </span>
                  </div>
                  <p className="result-detail">{result.점검내용}</p>
                </div>
              ))}
            </div>
          </div>
        )}
      </div>
    </main>
  );
}

export default App;
