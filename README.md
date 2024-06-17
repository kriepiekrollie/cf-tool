# cf
cf-tool is a command line interface tool for codeforces.

It is a work in progress.

Heavily inspired by [xalanq's cf-tool](https://github.com/xalanq/cf-tool/).


How i want this tool to work:
```
Examples:
  cf login
  
  cf template add           Add a new template by interacting through stdio.
  cf template set           Set default template by interacting through stdio.
  cf template set cpp       Set the template with alias "cpp" to be default template.
  cf template delete        Delete a template by interacting through stdio.
  cf template delete java   Delete the template with alias "java".
  
  cf parse 1976             Parse the problems from contest 1976
  cf parse --contest 1976   -
  cf parse -c 1976          -
  cf parse --gym 105224     Parse the problems from gym 105224
  cf parse -g 105224        -
  
  cf gen                    Generate a source file from default template.
  cf gen py                 Generate a source file from template with alias "py".

  cf test                   Test source file in current directory with parsed samples.
                            This will find pairs of files of the form ({}.in, {}.out) in 
                            the current directory and then test the users source on these
                            sample testcases.

  cf submit                 Submit source file in current directory to CF. This will try 
                            to deduce the contest and problem from the folder you are in. 
                            It will only work if the user is in a directory of the form
                                "<cf_root>/{contest,gym}/{contest_id}/{problem_id}/"

                            If there is more than one source file in the current
                            directory that matches one of the users templates, then it 
                            will prompt the user to specify.

  cf source                 Takes the user to the github page for this project.

  cf help                   Shows some help.
  cf help <subcommand>      More specific help.

```
