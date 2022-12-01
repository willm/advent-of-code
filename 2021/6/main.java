import java.util.*;
import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.stream.Stream;

class Fish {
    private int _daysLeft;
    public Fish() {
        _daysLeft = 8;
    }
    public Fish(int daysLeft) {
        _daysLeft = daysLeft;
    }
    public Fish tick() {
        if (this._daysLeft == 0) {
            this._daysLeft = 6;
            return new Fish();
        }
        this._daysLeft -= 1;
        return null;
    }

    public int getDaysLeft() {
        return this._daysLeft;
    }

    public String toString() {
        return Integer.toString(_daysLeft);
    }
}

class LaternFishSimulation {
    /*private static List<Fish> run(List<Fish> generation, days) {
        if (days == 0) {
            return generation;
        }

    }*/
    public static List<Fish> run(
        String path, int days
    ) {
        List<Fish> school = FishFile.parse(path);
        for (int day = 0; day < days; day++) {
            List<Fish> newSchool = new ArrayList<Fish>();
            for(Fish fish: school) {
                Fish newFish = fish.tick();
                if (newFish != null) {
                    newSchool.add(newFish);
                }
            }
            school.addAll(newSchool);
        }
        return school;
    }
}

class FishFile {
    public static List<Fish> parse(String path) {
        StringBuilder ages = new StringBuilder();

        try (Stream<String> stream = Files.lines(Paths.get(path), StandardCharsets.UTF_8))
        {
            stream.forEach(s -> ages.append(s));
        }
        catch (IOException e)
        {
            e.printStackTrace();
        }

        List<Fish> school = new ArrayList<Fish>();
        for (String age: ages.toString().split(",")) {
            school.add(new Fish(Integer.parseInt(age.replace("\n", ""))));
        }
        return school;
    }
}

public class Main {
    public static void main(String[] args) {
        Tests.run();
        System.out.println(LaternFishSimulation.run("input.txt", 80).size());
    }
}

class Tests {
    private static void whenNotReadyToSpawn() {
        Fish fish = new Fish();
        Fish newFish = fish.tick();
        assert newFish == null;
    }

    private static void testRegeneratesAt6() {
        Fish fish = new Fish();
        for (int i = 0; i < 8; i++) {
            fish.tick();
        }
        Fish newFish = fish.tick();
        assert fish.getDaysLeft() == 6: fish.getDaysLeft();
        assert newFish != null;
    }

    private static void testInput() {
        List<Fish> population = LaternFishSimulation.run(
            "input-test.txt", 18
        );
        assert population.size() == 26;

        population = LaternFishSimulation.run(
            "input-test.txt", 80
        );
        assert population.size() == 5934;

        /* Part 2...
         * population = LaternFishSimulation.run(
            "input-test.txt", 256
        );
        assert population.size() == 26984457539; */

    }
    public static void run() {
        whenNotReadyToSpawn();
        testRegeneratesAt6();
        testInput();
    }
}
