open Base

module XY = struct
  type t = int * int [@@deriving compare, sexp_of, hash]
end
