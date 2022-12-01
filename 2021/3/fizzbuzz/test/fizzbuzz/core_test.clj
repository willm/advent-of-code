(ns fizzbuzz.core-test
  (:require [clojure.test :refer :all]
            [fizzbuzz.core :refer :all]))
(defn fizzbuzz [x]
  (cond
    (= (mod x 15) 0) "fizzbuzz"
    (= (mod x 3) 0) "fizz"
    (= (mod x 5) 0) "buzz"
    :else (.toString x)))

(deftest a-test
  (testing "1 is 1"
    (is (= (fizzbuzz 1) "1"))))
  (testing "3 is fizz"
    (is (= (fizzbuzz 3) "fizz")))
  (testing "5 is buzz"
    (is (= (fizzbuzz 5) "buzz")))
  (testing "15 is fizzbuzz"
    (is (= (fizzbuzz 15) "fizzbuzz")))
