import React, { useState } from "react";
import { useLocale } from "../locale";

function BackupPage() {
  const [backup, setBackup] = useState("");
  const locale = useLocale();

  const handleGet = () => {
    fetch("http://localhost:8080/backup", { credentials: "include" })
      .then((response) => {
        if (response.ok) {
          return response.text();
        }
        throw new Error("Failed to fetch backup");
      })
      .then((data) => setBackup(data))
      .catch((error) => console.log(error.message));
  };

  const handleDownload = () => {
    const element = document.createElement("a");
    const file = new Blob([backup], { type: 'text/plain' });
    element.href = URL.createObjectURL(file);
    element.download = "backup.sql";
    document.body.appendChild(element);
    element.click();
  };

  return (
    <div className="p-4">
      <div className="flex mb-10 gap-10 items-center">
        <h1 className="text-2xl font-bold mb-4">{`${locale.backupTitle}`}</h1>
        <button
          className="bg-blue-500 text-white px-4 py-2 rounded"
          onClick={handleGet}
        >
          {`${locale.requestBackup}`}
        </button>
        <button
          className="bg-green-500 text-white px-4 py-2 rounded disabled:opacity-70"
          onClick={handleDownload}
          disabled={!backup}
        >
      {`${locale.downloadBackup}`}
        </button>
      </div>
      <textarea
        className="w-full h-96 p-2 border border-gray-300"
        readOnly
        value={backup}
      />
    </div>
  );
}

export default BackupPage;
