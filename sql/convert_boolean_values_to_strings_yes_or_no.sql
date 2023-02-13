SELECT bool,
  CASE bool
    WHEN TRUE THEN 'Yes'
    ELSE 'No'
  END
  as res
FROM booltoword;
