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

# Ch 2 (pages 40 - 67)

This section of chapter 2 covers query languages and touches on modeling data in graph databases. We also covered more information about what model fits varying application needs. There is also discussion about how all 3 models (graph, relational, document) are useful today and becoming more and more similar. FTS (full text search) is mentioned but not covered in this chapter as well as custom models used in gnome and particle research. In this section I found the graph database examples super interesting for solving applications solving problems for data with inter-connected, arbitrary relationships. I also find the merging of the 3 models (graph, relational, document) interesting. For example, a relation db such as PostGres with Json document support.