# FIBBOT GITHUB ACTION DOCUMENTATION
## Overview:
 This project's aim was to  develop  a GitHub Action in Rust that scans pull request content for numbers, calculates their Fibonacci numbers, and posts a comment with the results. 

 ## Features
 - it can collect the content of a PR
 -  it can detect numbers from that content
 -  it can calculate the fibonnacci of each number if it's not above the Max_threshold variable
 -  it can the post the result of the fibo calculation as a comment in the PR

## How To Use It

## PREREQUISITES

- a GitHub repository with Pull Request
- Github actions enabled in your Repository
## Steps
- fork or clone this repository
- create a workflow file add this piece of code to your worflow file
  ```yml
   - name: Run Hello World Rust Action && Argument Parsing Action
        uses: benie-joy-possi/fibbot@v1
        with:
          enable_fib: 'true'
          max_threshold: '1000'
          pr_number: ${{ github.event.pull_request.number }}
          github_token: ${{ secrets.GITHUB_TOKEN }}
          owner: ${{ github.repository_owner }}
          repository: ${{ github.repository }}
  ```
- commit the changes
- and make sure you set the **enable_fib** to *true* and the **Max_threshold** to *1000*
- enjoy the testing !
