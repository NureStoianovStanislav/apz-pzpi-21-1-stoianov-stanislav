import React, { useEffect, useState } from "react";
import { Link, useNavigate } from "react-router-dom";
import { useLocale } from "../locale";
import Library from "./Library";

function LibrariesPage() {
  const [libraries, setLibraries] = useState([]);
  const navigate = useNavigate();
  const locale = useLocale();

  useEffect(() => {
    fetch("http://localhost:8080/libraries")
      .then((response) => {
        if (response.status === 401 || response.status === 403) {
          navigate("/login");
        }
        if (response.ok) {
          return response.json();
        }
        throw new Error("Failed to fetch libraries");
      })
      .then((data) => setLibraries(data))
      .catch((error) => console.log(error.message));
  }, [navigate]);

  return (
    <div className="p-4">
      <div className="flex gap-10 items-center">
        <h1 className="text-2xl font-bold mb-4">{locale.librariesTitle}</h1>
        <div className="mb-4">
          <Link
            to="/new-library"
            className="bg-green-500 text-white px-4 py-2 rounded mr-2"
          >
            {locale.addLibrary}
          </Link>
        </div>
      </div>
      <div className="grid grid-cols-3 gap-10">
        {libraries.map((library) => (
          <Library library={library} />
        ))}
      </div>
    </div>
  );
}

export default LibrariesPage;
