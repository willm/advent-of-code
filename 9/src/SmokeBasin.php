<?php declare(strict_types=1);

final class Window {
    private array $window;
    function __construct($height_map, $x, $y) {
        $window_x = range($x -1, $x + 1);
        $window_y = range($y -1, $y + 1);
        $this->window = array_map(function ($x) use($window_y, $height_map) {
            return array_map(function ($y) use ($x, $height_map): ?int {
                if ($x < 0 || $y < 0 || $x > count($height_map) - 1 || $y > count($height_map) - 1) {
                    return null;
                }
                return $height_map[$x][$y];
            }, $window_y);
        }, $window_x);
    }

    public function get(): array {
        return $this->window;
    }

    public function is_low_point(): bool
    {
        $mid = $this->window[1][1];
        $left = $this->window[1][2];
        $right = $this->window[1][0];
        $up = $this->window[0][1];
        $down = $this->window[2][1];
        $real_points = array_filter(
            array($mid, $left, $right, $up, $down),
            function($point) { return !is_null($point); }
        );
        return count($real_points) > 0 && $mid == min($real_points);
    }
}

final class HeightMap {
    public static function from_file(string $path): array {
        $lines = file($path);
        return array_map(function ($line): array {
            $line = str_replace("\n", "", $line);
            return array_map(function($char): int {
                    return (int) $char;
                },
                str_split($line)
            );
        }, $lines);
    }
}

final class SmokeBasin
{
    public static function get_low_points_from_file(string $path): array
    {
        $height_map = HeightMap::from_file($path);
        return SmokeBasin::get_low_points($height_map);
    }
    public static function get_low_points(array $height_map): array {
        $points = array();
        for ($x = 0; $x < count($height_map); $x++) {
            for ($y = 0; $y < count($height_map); $y++) {
                $window = new Window($height_map, $x, $y);
                if ($window->is_low_point($window)) {
                    array_push($points, array($x, $y));
                }
            }
        }
        return $points;
    }
}
