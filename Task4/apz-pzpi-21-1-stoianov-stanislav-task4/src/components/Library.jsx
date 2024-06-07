import { Link } from "react-router-dom";
import { useLocale } from "../locale";

function Library({ library }) {
  const locale = useLocale();

  return (
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
  );
}

export default Library;
