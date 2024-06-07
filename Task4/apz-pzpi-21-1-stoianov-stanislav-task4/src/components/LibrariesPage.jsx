import React, { useEffect, useState } from "react";
import { Link, useNavigate } from "react-router-dom";
import { useLocale } from "../locale";

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
          <div key={library.id} className="bg-gray-100 rounded-lg p-4">
            <Link to={`/libraries/${library.id}`} className="block">
              <h2 className="font-semibold text-lg mb-2">{library.name}</h2>
              <p className="text-gray-600 mb-2">{`${locale.address}: ${library.address}`}</p>
              <div className="flex justify-between">
                <p className="text-gray-700">{`${locale.dailyRate}: ${library.dailyRate}`}</p>
                <p className="text-gray-700">{`${locale.overdueRate}: ${library.overdueRate}`}</p>
              </div>
              <p className="text-gray-700">{`${locale.currency}: ${library.currency}`}</p>
            </Link>
          </div>
        ))}
      </div>
    </div>
  );
}

export default LibrariesPage;
