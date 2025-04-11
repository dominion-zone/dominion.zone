;(theory Vectors :sorts ((Vector 1))

;    :funs (
;        (empty () (Vector T))
;        (push ((Vector T) T) (Vector T))
;        (get ((Vector T) Int) T)
;        (length ((Vector T)) Int)
;    )

;    :definition (
        ; Аксиома: длина пустого вектора равна 0
;        (= (length (empty)) 0)

        ; Аксиома: длина вектора после добавления элемента увеличивается на 1
;        (forall ((v (Vector T)) (x T))
;            (= (length (push v x)) (+ 1 (length v))))

;        ; Аксиома: доступ к последнему элементу вектора
;        (forall ((v (Vector T)) (x T))
;            (= (get (push v x) (length v)) x))
;
        ; Аксиома: доступ к любому другому элементу сохраняется при добавлении
;        (forall ((v (Vector T)) (x T) (i Int))
;            (=> (and (>= i 0) (< i (length v)))
;                (= (get (push v x) i) (get v i))))
;    )
;)

(set-logic ALL)

(declare-datatype Vector
  (par (T) ((Vector (data (Array Int T)) (length Int)))))

;(define-fun is-vector-correct ((v (Vector T))) Bool
;  (and
;    (>= (length v) 0)
;    (forall ((i Int))
;      (= (exists ((val T)) (= (select (data v) i) val))
;          (and (>= i 0) (< i (length v)))))))

(declare-datatypes (
  (|0x0000000000000000000000000000000000000000000000000000000000000001::option::Option| 1))
  (
  (par (T0) ((|0x0000000000000000000000000000000000000000000000000000000000000001::option::Option| (vec (Vector T0)))))))

(declare-datatypes (
  (|0x0000000000000000000000000000000000000000000000000000000000000001::ascii::String| 0)
  (|0x0000000000000000000000000000000000000000000000000000000000000001::ascii::Char| 0))
  (
  ((|0x0000000000000000000000000000000000000000000000000000000000000001::ascii::String| (bytes (Vector Int))))
  ((|0x0000000000000000000000000000000000000000000000000000000000000001::ascii::Char| (byte Int)))))


(declare-datatypes (
  (|0x0000000000000000000000000000000000000000000000000000000000000001::bit_vector::BitVector| 0))
  (
  ((|0x0000000000000000000000000000000000000000000000000000000000000001::bit_vector::BitVector| (length Int) (bit_field (Vector Bool))))))


(declare-datatypes (
  (|0x0000000000000000000000000000000000000000000000000000000000000001::fixed_point32::FixedPoint32| 0))
  (
  ((|0x0000000000000000000000000000000000000000000000000000000000000001::fixed_point32::FixedPoint32| (value Int)))))



(declare-datatypes (
  (|0x0000000000000000000000000000000000000000000000000000000000000001::string::String| 0))
  (
  ((|0x0000000000000000000000000000000000000000000000000000000000000001::string::String| (bytes (Vector Int))))))

(declare-datatypes (
  (|0x0000000000000000000000000000000000000000000000000000000000000001::type_name::TypeName| 0))
  (
  ((|0x0000000000000000000000000000000000000000000000000000000000000001::type_name::TypeName| (name |0x0000000000000000000000000000000000000000000000000000000000000001::ascii::String|)))))







(declare-datatypes (
  (|0x0000000000000000000000000000000000000000000000000000000000000001::uq32_32::UQ32_32| 0))
  (
  ((|0x0000000000000000000000000000000000000000000000000000000000000001::uq32_32::UQ32_32| (pos0 Int)))))

(declare-datatypes (
  (|0x0000000000000000000000000000000000000000000000000000000000000001::uq64_64::UQ64_64| 0))
  (
  ((|0x0000000000000000000000000000000000000000000000000000000000000001::uq64_64::UQ64_64| (pos0 Int)))))