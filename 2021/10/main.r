require(testthat)

blocks.data <- data.frame(
        start = c("(", "["),
        end   = c(")", "]")
)
print(blocks.data[, "start"])
print("(" %in% blocks.data$start)
print(")" %in% blocks.data$start)


is_valid <- function(input) {
        open_blocks <- list()
        characters <- strsplit(input, "")
        for (i in 1:length(characters)) {
                if (characters[i] %in% blocks.data$start) {
                        print(characters[i])
                        open_blocks.append(characters[i])
                } else {
                        row <- characters[blocks.data$end == characters[i]][0]
                        if (open_blocks[length(open_blocks)] != row$start) {
                                return(FALSE)
                        }
                }
        }
        TRUE
}

# keep list of all the openers
# when a close comes look back 1 to check it closes it


test_that("a simple block is valid", {
        expect_equal(is_valid("()"), TRUE)
})

test_that("an invalid block is invalid", {
        expect_equal(is_valid("([)"), FALSE)
})