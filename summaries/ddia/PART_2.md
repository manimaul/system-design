# Ch 2 (pages 27 - 40)

This chapter covers a history of data models and query languages as well as hiding complexity behind APIs. The Relational Model is compared to the Document Model including how the data is layed out. The document model (nosql) is not new. The document model is compared to the network model and hierarchical model which were alternatives to the relational model in the 70s and 80s. One of the major disadvantages of the relational model is the awkwardness of representing data in object oriented code. (impedance mismatch) JSON representation has better locality. Many to one and many to many relationships are discussed as an advantage of the relational model and storing and id rather than a particular type is recommended where flexibility for change may be required. Updating from a particular type to an id and removing duplication is "normalizing" the database. 

Document Db vs Relational:
Document - flexible schema (schema-on-read)
 - poor support for joins
 - OK many to one
 - BAD many to many
Relational - strict schema (schema-on-write)
 - GOOD many to one
 - GOOD many to many

Terms:
`object relational impedance mismatch`: technical difficulties representing RDBMS data in object oriented programming 