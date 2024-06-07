import { useState, useEffect } from "react";

export function useLocale() {
  let [locale, setLocale] = useState({});
  const language = localStorage.getItem("language") || "en";
  useEffect(() => {
    fetch(`/locales/${language}/dictionary.json`)
      .then((res) => res.json())
      .then((res) => setLocale(res));
  }, []);
  return locale;
}
