import React from "react";
import { Link } from "react-router-dom";
import { useLocale } from "../locale";

function Header() {
  const setLocale = (language) => () => {
    localStorage.setItem("language", language);
    window.location.reload();
  };
  const locale = useLocale();

  return (
    <header className="bg-gray-800 text-white p-4">
      <nav className="container mx-auto flex justify-between items-center">
        <div className="flex justify-between w-screen">
          <Link to="/libraries" className="mr-4">
            <h1 className="text-xl font-bold">{`${locale.libraries}`}</h1>
          </Link>
          <Link to="/backup" className="mr-4">
            {`${locale.backup}`}
          </Link>
          <Link to="/login" className="mr-4">
            {`${locale.login}`}
          </Link>
          <Link to="/signup" className="mr-4">
            {`${locale.signup}`}
          </Link>
          <div className="flex gap-5">
            <button onClick={setLocale("en")}>en</button>
            <button onClick={setLocale("ua")}>ua</button>
          </div>
        </div>
      </nav>
    </header>
  );
}

export default Header;
