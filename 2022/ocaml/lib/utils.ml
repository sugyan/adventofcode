open Base

module XY = struct
  type t = int * int [@@deriving compare, sexp_of, hash]
end

module XYZ = struct
  type t = int * int * int [@@deriving compare, sexp_of, hash]
end

let scan t ~init ~f =
  List.folding_map t ~init ~f:(fun acc x -> f acc x |> fun y -> (y, y))

let rec pairs = function
  | [] | [ _ ] -> []
  | a :: b :: tl -> (a, b) :: pairs (b :: tl)
